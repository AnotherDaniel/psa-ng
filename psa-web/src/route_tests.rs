// Tests for web server REST API endpoints.
//
// These tests use axum_test to exercise the router with a real AppState
// backed by wiremock (for PSA API) and :memory: SQLite (for persistence).

#[cfg(test)]
mod tests {
    use crate::db::Database;
    use crate::routes::create_router;
    use crate::state::AppState;
    use axum_test::TestServer;
    use chrono::Utc;
    use psa_api::auth::{OAuthClient, TokenData};
    use psa_api::client::PsaClient;
    use psa_api::config::{AppConfig, ElectricityConfig, PsaConfig, ServerConfig};
    use std::path::Path;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicU64, Ordering};
    use tokio::sync::Mutex;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    fn mock_auth(mock_uri: &str) -> PsaClient {
        static COUNTER: AtomicU64 = AtomicU64::new(0);
        let id = COUNTER.fetch_add(1, Ordering::Relaxed);
        let dir = std::env::temp_dir().join(format!("psa-ng-route-test-{id}"));
        std::fs::create_dir_all(&dir).unwrap();
        let token_path = dir.join("mock_token.json");
        let token = TokenData {
            access_token: "test_access_token".to_string(),
            refresh_token: "test_refresh_token".to_string(),
            token_type: "Bearer".to_string(),
            expires_at: Utc::now() + chrono::Duration::hours(1),
            scope: None,
        };
        std::fs::write(&token_path, serde_json::to_string(&token).unwrap()).unwrap();

        let auth = OAuthClient::new(
            "test_id".to_string(),
            "test_secret".to_string(),
            "peugeot".to_string(),
            Some(token_path),
        );
        PsaClient::new(auth, Some(mock_uri.to_string()))
    }

    fn test_config() -> AppConfig {
        AppConfig {
            psa: PsaConfig {
                client_id: "test".to_string(),
                client_secret: "secret".to_string(),
                brand: "peugeot".to_string(),
                api_base_url: "http://localhost".to_string(),
                token_file: None,
            },
            server: ServerConfig::default(),
            electricity: ElectricityConfig {
                price_per_kwh: 0.15,
                currency: "EUR".to_string(),
                ..Default::default()
            },
        }
    }

    async fn setup(mock_server: &MockServer) -> TestServer {
        let dir = std::env::temp_dir().join(format!("psa-ng-route-cfg-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let config_path = dir.join("test_config.toml");

        let config = test_config();
        config.save(&config_path).unwrap();

        let state = Arc::new(AppState {
            psa_client: Mutex::new(mock_auth(&mock_server.uri())),
            config: Mutex::new(config),
            config_path,
            db: Arc::new(Database::open(Path::new(":memory:")).unwrap()),
        });
        let router = create_router(state);
        TestServer::new(router).unwrap()
    }

    // ── Vehicle status endpoint ──────────────────────────────────────

    // [utest->req~vehicle-status-endpoint~1]
    #[tokio::test]
    async fn test_get_vehicles_endpoint() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/user/vehicles"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "_embedded": {
                    "vehicles": [
                        {
                            "id": "v1",
                            "vin": "VF3XXXXXXXXXXXXX",
                            "brand": "Peugeot",
                            "label": "e-208"
                        }
                    ]
                }
            })))
            .mount(&mock_server)
            .await;

        let server = setup(&mock_server).await;
        let resp = server.get("/api/vehicles").await;
        resp.assert_status_ok();
        let body: serde_json::Value = resp.json();
        assert!(body.is_array());
        assert_eq!(body[0]["vin"], "VF3XXXXXXXXXXXXX");
    }

    // [utest->req~vehicle-status-endpoint~1]
    #[tokio::test]
    async fn test_get_vehicle_status_endpoint() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/user/vehicles/v1/status"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "updatedAt": "2026-01-15T10:30:00Z",
                "energy": [{
                    "type": "Electric",
                    "level": 80.0,
                    "autonomy": 250.0,
                    "charging": { "status": "Disconnected", "chargingMode": "No" }
                }],
                "odometer": { "mileage": 12000.0 }
            })))
            .mount(&mock_server)
            .await;

        let server = setup(&mock_server).await;
        let resp = server.get("/api/vehicles/v1/status").await;
        resp.assert_status_ok();
        let body: serde_json::Value = resp.json();
        assert!(body["energy"].is_array());
    }

    // ── Wakeup endpoint ──────────────────────────────────────────────

    // [utest->req~wakeup-endpoint~1]
    #[tokio::test]
    async fn test_wakeup_endpoint() {
        let mock_server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/user/vehicles/v1/callbacks"))
            .respond_with(ResponseTemplate::new(200))
            .mount(&mock_server)
            .await;

        let server = setup(&mock_server).await;
        let resp = server.post("/api/vehicles/v1/wakeup").await;
        resp.assert_status_ok();
        let body: serde_json::Value = resp.json();
        assert_eq!(body["status"], "ok");
    }

    // ── Charge control endpoint ──────────────────────────────────────

    // [utest->req~charge-control-endpoint~1]
    #[tokio::test]
    async fn test_charge_start_endpoint() {
        let mock_server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/user/vehicles/v1/callbacks"))
            .respond_with(ResponseTemplate::new(200))
            .mount(&mock_server)
            .await;

        let server = setup(&mock_server).await;
        let resp = server
            .post("/api/vehicles/v1/charge")
            .json(&serde_json::json!({"start": true}))
            .await;
        resp.assert_status_ok();
        let body: serde_json::Value = resp.json();
        assert_eq!(body["status"], "ok");
    }

    // [utest->req~charge-control-endpoint~1]
    #[tokio::test]
    async fn test_charge_threshold_endpoint() {
        let mock_server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/user/vehicles/v1/callbacks"))
            .respond_with(ResponseTemplate::new(200))
            .mount(&mock_server)
            .await;

        let server = setup(&mock_server).await;
        let resp = server
            .post("/api/vehicles/v1/charge/threshold")
            .json(&serde_json::json!({"percentage": 80}))
            .await;
        resp.assert_status_ok();
    }

    // [utest->req~charge-control-endpoint~1]
    #[tokio::test]
    async fn test_charge_schedule_endpoint() {
        let mock_server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/user/vehicles/v1/callbacks"))
            .respond_with(ResponseTemplate::new(200))
            .mount(&mock_server)
            .await;

        let server = setup(&mock_server).await;
        let resp = server
            .post("/api/vehicles/v1/charge/schedule")
            .json(&serde_json::json!({"hour": 6, "minute": 0}))
            .await;
        resp.assert_status_ok();
    }

    // ── Preconditioning endpoint ─────────────────────────────────────

    // [utest->req~preconditioning-endpoint~1]
    #[tokio::test]
    async fn test_preconditioning_endpoint() {
        let mock_server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/user/vehicles/v1/callbacks"))
            .respond_with(ResponseTemplate::new(200))
            .mount(&mock_server)
            .await;

        let server = setup(&mock_server).await;
        let resp = server
            .post("/api/vehicles/v1/preconditioning")
            .json(&serde_json::json!({"start": true}))
            .await;
        resp.assert_status_ok();
        let body: serde_json::Value = resp.json();
        assert_eq!(body["status"], "ok");
    }

    // ── Door lock endpoint ───────────────────────────────────────────

    // [utest->req~door-lock-endpoint~1]
    #[tokio::test]
    async fn test_door_lock_endpoint() {
        let mock_server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/user/vehicles/v1/callbacks"))
            .respond_with(ResponseTemplate::new(200))
            .mount(&mock_server)
            .await;

        let server = setup(&mock_server).await;
        let resp = server
            .post("/api/vehicles/v1/doors")
            .json(&serde_json::json!({"lock": true}))
            .await;
        resp.assert_status_ok();
        let body: serde_json::Value = resp.json();
        assert_eq!(body["status"], "ok");
    }

    // ── Lights and horn endpoint ─────────────────────────────────────

    // [utest->req~lights-horn-endpoint~1]
    #[tokio::test]
    async fn test_lights_endpoint() {
        let mock_server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/user/vehicles/v1/callbacks"))
            .respond_with(ResponseTemplate::new(200))
            .mount(&mock_server)
            .await;

        let server = setup(&mock_server).await;
        let resp = server
            .post("/api/vehicles/v1/lights")
            .json(&serde_json::json!({"duration": 10}))
            .await;
        resp.assert_status_ok();
    }

    // [utest->req~lights-horn-endpoint~1]
    #[tokio::test]
    async fn test_horn_endpoint() {
        let mock_server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/user/vehicles/v1/callbacks"))
            .respond_with(ResponseTemplate::new(200))
            .mount(&mock_server)
            .await;

        let server = setup(&mock_server).await;
        let resp = server
            .post("/api/vehicles/v1/horn")
            .json(&serde_json::json!({"count": 3}))
            .await;
        resp.assert_status_ok();
    }

    // ── Settings endpoint ────────────────────────────────────────────

    // [utest->req~settings-endpoint~1]
    #[tokio::test]
    async fn test_get_settings_endpoint() {
        let mock_server = MockServer::start().await;
        let server = setup(&mock_server).await;
        let resp = server.get("/api/settings").await;
        resp.assert_status_ok();
        let body: serde_json::Value = resp.json();
        assert_eq!(body["price_per_kwh"], 0.15);
        assert_eq!(body["currency"], "EUR");
    }

    // [utest->req~settings-endpoint~1]
    #[tokio::test]
    async fn test_update_settings_endpoint() {
        let mock_server = MockServer::start().await;
        let server = setup(&mock_server).await;
        let resp = server
            .post("/api/settings")
            .json(&serde_json::json!({
                "price_per_kwh": 0.25,
                "currency": "USD"
            }))
            .await;
        resp.assert_status_ok();

        // Verify the change persisted
        let resp2 = server.get("/api/settings").await;
        let body: serde_json::Value = resp2.json();
        assert_eq!(body["price_per_kwh"], 0.25);
        assert_eq!(body["currency"], "USD");
    }

    // ── Trips endpoint ───────────────────────────────────────────────

    // [utest->req~trips-endpoint~1]
    #[tokio::test]
    async fn test_get_trips_endpoint() {
        let mock_server = MockServer::start().await;
        let server = setup(&mock_server).await;
        let resp = server.get("/api/trips").await;
        resp.assert_status_ok();
        let body: serde_json::Value = resp.json();
        assert!(body.is_array());
        assert_eq!(body.as_array().unwrap().len(), 0);
    }

    // ── Charging sessions endpoint ───────────────────────────────────

    // [utest->req~charging-sessions-endpoint~1]
    #[tokio::test]
    async fn test_get_charging_sessions_endpoint() {
        let mock_server = MockServer::start().await;
        let server = setup(&mock_server).await;
        let resp = server.get("/api/charging-sessions").await;
        resp.assert_status_ok();
        let body: serde_json::Value = resp.json();
        assert!(body.is_array());
        assert_eq!(body.as_array().unwrap().len(), 0);
    }

    // ── Security tests ──────────────────────────────────────────────

    async fn setup_with_token(mock_server: &MockServer, token: &str) -> TestServer {
        let dir = std::env::temp_dir().join(format!("psa-ng-route-sec-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let config_path = dir.join("test_config.toml");

        let mut config = test_config();
        config.server.api_token = Some(token.to_string());
        config.save(&config_path).unwrap();

        let state = Arc::new(AppState {
            psa_client: Mutex::new(mock_auth(&mock_server.uri())),
            config: Mutex::new(config),
            config_path,
            db: Arc::new(Database::open(Path::new(":memory:")).unwrap()),
        });
        let router = create_router(state);
        TestServer::new(router).unwrap()
    }

    // [utest->req~api-bearer-auth~1]
    #[tokio::test]
    async fn test_api_rejects_missing_token() {
        let mock_server = MockServer::start().await;
        let server = setup_with_token(&mock_server, "secret123").await;
        let resp = server.get("/api/vehicles").await;
        resp.assert_status(axum::http::StatusCode::UNAUTHORIZED);
    }

    // [utest->req~api-bearer-auth~1]
    #[tokio::test]
    async fn test_api_rejects_wrong_token() {
        let mock_server = MockServer::start().await;
        let server = setup_with_token(&mock_server, "secret123").await;
        let resp = server
            .get("/api/vehicles")
            .add_header(
                axum::http::header::AUTHORIZATION,
                "Bearer wrong_token"
                    .parse::<axum::http::HeaderValue>()
                    .unwrap(),
            )
            .await;
        resp.assert_status(axum::http::StatusCode::UNAUTHORIZED);
    }

    // [utest->req~api-bearer-auth~1]
    #[tokio::test]
    async fn test_api_accepts_correct_token() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/user/vehicles"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "_embedded": { "vehicles": [] }
            })))
            .mount(&mock_server)
            .await;

        let server = setup_with_token(&mock_server, "secret123").await;
        let resp = server
            .get("/api/vehicles")
            .add_header(
                axum::http::header::AUTHORIZATION,
                "Bearer secret123"
                    .parse::<axum::http::HeaderValue>()
                    .unwrap(),
            )
            .await;
        resp.assert_status_ok();
    }

    // [utest->req~security-headers~1]
    #[tokio::test]
    async fn test_security_headers_present() {
        let mock_server = MockServer::start().await;
        let server = setup(&mock_server).await;
        let resp = server.get("/").await;
        assert_eq!(resp.header("X-Content-Type-Options"), "nosniff");
        assert_eq!(resp.header("X-Frame-Options"), "DENY");
        assert!(!resp.header("Content-Security-Policy").is_empty());
        assert!(!resp.header("Referrer-Policy").is_empty());
    }

    // [utest->req~sanitized-errors~1]
    #[tokio::test]
    async fn test_error_responses_sanitized() {
        let mock_server = MockServer::start().await;
        // Return a 500 error from upstream with internal details
        Mock::given(method("GET"))
            .and(path("/user/vehicles"))
            .respond_with(
                ResponseTemplate::new(500).set_body_string("Internal error at /var/app/secrets"),
            )
            .mount(&mock_server)
            .await;

        let server = setup(&mock_server).await;
        let resp = server.get("/api/vehicles").await;
        let body = resp.text();
        assert!(!body.contains("/var/app"));
        assert!(!body.contains("secrets"));
    }

    // [utest->req~html-output-escaping~1]
    #[test]
    fn test_html_escaping() {
        use crate::templates::escape_html;
        assert_eq!(
            escape_html("<script>alert(1)</script>"),
            "&lt;script&gt;alert(1)&lt;/script&gt;"
        );
        assert_eq!(escape_html("a&b"), "a&amp;b");
        assert_eq!(escape_html(r#"x"y'z"#), "x&quot;y&#x27;z");
    }
}
