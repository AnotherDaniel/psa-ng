use crate::auth::OAuthClient;
use crate::error::{ApiErrorResponse, PsaError, Result};
use crate::models::{
    CallbackRequest, CallbackResponse, RemoteActionResponse, RemoteCharging,
    RemoteChargingPreferences, RemoteChargingSchedule, RemoteCommand, RemoteDoor, RemoteHorn,
    RemoteLights, RemotePrecondAirCon, RemotePreconditioning, RemoteWakeUp, Vehicle, VehicleStatus,
    VehiclesResponse,
};
use reqwest::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use tracing::{debug, info};

const DEFAULT_BASE_URL: &str = "https://api.groupe-psa.com/connectedcar/v4";

/// High-level client for the PSA Connected Car v4 REST API.
///
/// Wraps an [`OAuthClient`] for authentication and exposes typed methods
/// for vehicle queries and remote commands.
pub struct PsaClient {
    auth: OAuthClient,
    base_url: String,
    http: reqwest::Client,
    // [impl->req~callback-registration~1]
    callback_id: Option<String>,
}

impl PsaClient {
    /// Create a new API client with the given auth provider and optional base URL override.
    pub fn new(auth: OAuthClient, base_url: Option<String>) -> Self {
        let http = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .expect("Failed to build HTTP client");

        Self {
            auth,
            base_url: base_url.unwrap_or_else(|| DEFAULT_BASE_URL.to_string()),
            http,
            callback_id: None,
        }
    }

    /// Build the `Authorization: Bearer <token>` header value.
    async fn auth_header(&mut self) -> Result<String> {
        let token = self.auth.get_valid_token().await?;
        Ok(format!("Bearer {token}"))
    }

    /// Perform an authenticated GET request against the API.
    // [impl->req~rate-limit-handling~1]
    async fn get(&mut self, path: &str) -> Result<reqwest::Response> {
        let url = format!("{}{}", self.base_url, path);
        let auth = self.auth_header().await?;

        debug!("GET {}", url);
        let response = self
            .http
            .get(&url)
            .header(AUTHORIZATION, auth)
            .header(ACCEPT, "application/hal+json")
            .send()
            .await?;

        self.check_response(response).await
    }

    // [impl->req~remote-command-schema~1]
    /// POST a remote command to the API using the correct endpoint and schema.
    async fn post_remote(
        &mut self,
        vehicle_id: &str,
        callback_id: &str,
        body: &RemoteCommand,
    ) -> Result<RemoteActionResponse> {
        let url = format!(
            "{}/user/vehicles/{}/callbacks/{}/remotes",
            self.base_url, vehicle_id, callback_id
        );
        let auth = self.auth_header().await?;

        debug!("POST {}", url);
        let response = self
            .http
            .post(&url)
            .header(AUTHORIZATION, auth)
            .header(CONTENT_TYPE, "application/json")
            .header(ACCEPT, "application/hal+json")
            .json(body)
            .send()
            .await?;

        let response = self.check_response(response).await?;
        let action: RemoteActionResponse = response.json().await?;
        Ok(action)
    }

    /// POST a JSON body to an API path (for callback registration).
    async fn post_json(
        &mut self,
        path: &str,
        body: &impl serde::Serialize,
    ) -> Result<reqwest::Response> {
        let url = format!("{}{}", self.base_url, path);
        let auth = self.auth_header().await?;

        debug!("POST {}", url);
        let response = self
            .http
            .post(&url)
            .header(AUTHORIZATION, auth)
            .header(CONTENT_TYPE, "application/json")
            .header(ACCEPT, "application/hal+json")
            .json(body)
            .send()
            .await?;

        self.check_response(response).await
    }

    // [impl->req~api-error-parsing~1]
    // [impl->req~rate-limit-handling~1]
    /// Check an HTTP response for errors, parsing structured API errors and rate-limit headers.
    async fn check_response(&self, response: reqwest::Response) -> Result<reqwest::Response> {
        let status = response.status();

        if status.as_u16() == 429 {
            let retry_after = response
                .headers()
                .get("Retry-After")
                .and_then(|v| v.to_str().ok())
                .and_then(|v| v.parse::<u64>().ok())
                .unwrap_or(60);
            return Err(PsaError::RateLimited {
                retry_after_secs: retry_after,
            });
        }

        if !status.is_success() {
            let status_code = status.as_u16();
            let body = response.text().await.unwrap_or_default();
            let structured = serde_json::from_str::<ApiErrorResponse>(&body).ok();
            let detail = structured
                .as_ref()
                .map(|e| format!("[{}] {}", e.code, e.message))
                .unwrap_or_else(|| body.clone());
            return Err(PsaError::Api {
                status: status_code,
                detail,
                structured,
            });
        }

        Ok(response)
    }

    // [impl->req~vehicle-list~1]
    // [impl->req~api-pagination~1]
    /// Retrieve all vehicles for the authenticated user, following pagination.
    pub async fn get_vehicles(&mut self) -> Result<Vec<Vehicle>> {
        info!("Fetching vehicle list");
        let mut all_vehicles = Vec::new();
        let mut page_token: Option<String> = None;

        loop {
            let path = match &page_token {
                Some(token) => format!(
                    "/user/vehicles?pageSize=60&pageToken={}",
                    url::form_urlencoded::byte_serialize(token.as_bytes()).collect::<String>()
                ),
                None => "/user/vehicles?pageSize=60".to_string(),
            };

            let response = self.get(&path).await?;
            let data: VehiclesResponse = response.json().await?;

            if let Some(embedded) = data.embedded {
                all_vehicles.extend(embedded.vehicles);
            }

            // Follow pagination via next link
            let next_href = data
                .links
                .as_ref()
                .and_then(|l| l.next.as_ref())
                .and_then(|n| n.href.as_ref());

            if let Some(href) = next_href {
                // Extract pageToken from the next URL
                if let Some(pt) = extract_page_token(href) {
                    page_token = Some(pt);
                    continue;
                }
            }
            break;
        }

        Ok(all_vehicles)
    }

    // [impl->req~vehicle-status~1]
    /// Retrieve the current status of a vehicle
    pub async fn get_vehicle_status(&mut self, vehicle_id: &str) -> Result<VehicleStatus> {
        info!("Fetching status for vehicle {}", vehicle_id);
        let response = self
            .get(&format!("/user/vehicles/{vehicle_id}/status"))
            .await?;
        let status: VehicleStatus = response.json().await?;
        Ok(status)
    }

    // [impl->req~vehicle-wakeup~1]
    // [impl->req~remote-command-schema~1]
    /// Send a wakeup request to force the vehicle to report status
    pub async fn wakeup(&mut self, vehicle_id: &str) -> Result<()> {
        info!("Sending wakeup to vehicle {}", vehicle_id);
        let cbid = self.ensure_callback().await?;
        let cmd = RemoteCommand {
            wake_up: Some(RemoteWakeUp {
                action: "WakeUp".to_string(),
            }),
            ..Default::default()
        };
        self.post_remote(vehicle_id, &cbid, &cmd).await?;
        Ok(())
    }

    // [impl->req~charge-control~1]
    // [impl->req~remote-command-schema~1]
    /// Start or stop vehicle charging
    pub async fn set_charge(&mut self, vehicle_id: &str, start: bool) -> Result<()> {
        let action = if start { "start" } else { "stop" };
        info!("{}ing charge for vehicle {}", action, vehicle_id);
        let cbid = self.ensure_callback().await?;
        let cmd = RemoteCommand {
            charging: Some(RemoteCharging {
                immediate: Some(start),
                schedule: None,
                preferences: None,
            }),
            ..Default::default()
        };
        self.post_remote(vehicle_id, &cbid, &cmd).await?;
        Ok(())
    }

    // [impl->req~charge-threshold~1]
    // [impl->req~remote-command-schema~1]
    /// Set the charge threshold percentage
    pub async fn set_charge_threshold(&mut self, vehicle_id: &str, percentage: u8) -> Result<()> {
        info!(
            "Setting charge threshold to {}% for vehicle {}",
            percentage, vehicle_id
        );
        let cbid = self.ensure_callback().await?;
        let cmd = RemoteCommand {
            charging: Some(RemoteCharging {
                immediate: None,
                schedule: None,
                preferences: Some(RemoteChargingPreferences {
                    limit_soc: Some(percentage),
                }),
            }),
            ..Default::default()
        };
        self.post_remote(vehicle_id, &cbid, &cmd).await?;
        Ok(())
    }

    // [impl->req~charge-scheduling~1]
    // [impl->req~remote-command-schema~1]
    /// Set the scheduled charge stop hour
    pub async fn set_charge_schedule(
        &mut self,
        vehicle_id: &str,
        hour: u8,
        minute: u8,
    ) -> Result<()> {
        info!(
            "Setting charge schedule to {:02}:{:02} for vehicle {}",
            hour, minute, vehicle_id
        );
        let cbid = self.ensure_callback().await?;
        let cmd = RemoteCommand {
            charging: Some(RemoteCharging {
                immediate: None,
                schedule: Some(RemoteChargingSchedule {
                    next_delayed_time: format!("PT{hour}H{minute}M"),
                }),
                preferences: None,
            }),
            ..Default::default()
        };
        self.post_remote(vehicle_id, &cbid, &cmd).await?;
        Ok(())
    }

    // [impl->req~preconditioning-control~1]
    // [impl->req~remote-command-schema~1]
    /// Start or stop air conditioning preconditioning
    pub async fn set_preconditioning(&mut self, vehicle_id: &str, start: bool) -> Result<()> {
        let status = if start { "Activate" } else { "Deactivate" };
        info!("{}ing preconditioning for vehicle {}", status, vehicle_id);
        let cbid = self.ensure_callback().await?;
        let cmd = RemoteCommand {
            preconditioning: Some(RemotePreconditioning {
                air_conditioning: RemotePrecondAirCon {
                    status: status.to_string(),
                },
            }),
            ..Default::default()
        };
        self.post_remote(vehicle_id, &cbid, &cmd).await?;
        Ok(())
    }

    // [impl->req~door-lock-control~1]
    // [impl->req~remote-command-schema~1]
    /// Lock or unlock vehicle doors
    pub async fn set_door_lock(&mut self, vehicle_id: &str, lock: bool) -> Result<()> {
        let state = if lock { "Locked" } else { "Unlocked" };
        info!("Setting doors to {} for vehicle {}", state, vehicle_id);
        let cbid = self.ensure_callback().await?;
        let cmd = RemoteCommand {
            door: Some(RemoteDoor {
                state: state.to_string(),
            }),
            ..Default::default()
        };
        self.post_remote(vehicle_id, &cbid, &cmd).await?;
        Ok(())
    }

    // [impl->req~lights-horn-control~1]
    // [impl->req~remote-command-schema~1]
    /// Flash lights
    pub async fn flash_lights(&mut self, vehicle_id: &str, _duration: u32) -> Result<()> {
        info!("Flashing lights on vehicle {}", vehicle_id);
        let cbid = self.ensure_callback().await?;
        let cmd = RemoteCommand {
            lights: Some(RemoteLights { on: true }),
            ..Default::default()
        };
        self.post_remote(vehicle_id, &cbid, &cmd).await?;
        Ok(())
    }

    // [impl->req~lights-horn-control~1]
    // [impl->req~remote-command-schema~1]
    /// Honk the horn
    pub async fn honk_horn(&mut self, vehicle_id: &str, _count: u32) -> Result<()> {
        info!("Honking horn on vehicle {}", vehicle_id);
        let cbid = self.ensure_callback().await?;
        let cmd = RemoteCommand {
            horn: Some(RemoteHorn {
                state: "Activated".to_string(),
            }),
            ..Default::default()
        };
        self.post_remote(vehicle_id, &cbid, &cmd).await?;
        Ok(())
    }

    // [impl->req~callback-registration~1]
    /// Ensure a callback is registered, creating one if needed.
    /// Returns the callback ID.
    async fn ensure_callback(&mut self) -> Result<String> {
        if let Some(ref id) = self.callback_id {
            return Ok(id.clone());
        }
        let id = self.register_callback().await?;
        self.callback_id = Some(id.clone());
        Ok(id)
    }

    // [impl->req~callback-registration~1]
    /// Register a webhook callback with the PSA API.
    pub async fn register_callback(&mut self) -> Result<String> {
        info!("Registering callback with PSA API");
        let request = CallbackRequest {
            label: Some("psa-ng".to_string()),
            r#type: Some(vec!["Remote".to_string()]),
            callback: crate::models::CallbackConfig {
                webhook: Some(crate::models::WebhookConfig {
                    url: "https://localhost/callback".to_string(),
                    headers: None,
                }),
            },
        };

        let response = self.post_json("/user/callbacks", &request).await?;
        let cb: CallbackResponse = response.json().await?;
        cb.callback_id.ok_or_else(|| PsaError::Api {
            status: 0,
            detail: "Callback registration returned no ID".to_string(),
            structured: None,
        })
    }

    /// Set the callback ID to use for remote commands (e.g. loaded from config).
    pub fn set_callback_id(&mut self, id: String) {
        self.callback_id = Some(id);
    }

    /// Returns `true` if the underlying OAuth client holds a token.
    pub fn has_authentication(&self) -> bool {
        self.auth.has_token()
    }

    /// Mutable access to the underlying OAuth client (for token management).
    pub fn auth_mut(&mut self) -> &mut OAuthClient {
        &mut self.auth
    }
}

// [impl->req~api-pagination~1]
/// Extract the `pageToken` query parameter from a URL string.
fn extract_page_token(url: &str) -> Option<String> {
    url.split('?').nth(1).and_then(|query| {
        query.split('&').find_map(|param| {
            let (key, value) = param.split_once('=')?;
            if key == "pageToken" {
                Some(
                    url::form_urlencoded::parse(value.as_bytes())
                        .next()
                        .map(|(v, _)| v.into_owned())
                        .unwrap_or_else(|| value.to_string()),
                )
            } else {
                None
            }
        })
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::auth::OAuthClient;
    use chrono::Utc;
    use wiremock::matchers::{header, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    fn mock_auth() -> OAuthClient {
        use crate::auth::TokenData;
        use std::sync::atomic::{AtomicU64, Ordering};
        static COUNTER: AtomicU64 = AtomicU64::new(0);
        let id = COUNTER.fetch_add(1, Ordering::Relaxed);
        let dir = std::env::temp_dir().join(format!("psa-ng-client-test-{id}"));
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

        OAuthClient::new(
            "test_id".to_string(),
            "test_secret".to_string(),
            "peugeot".to_string(),
            Some(token_path),
        )
    }

    /// Create a client with a pre-set callback ID to avoid needing callback registration in every test.
    fn mock_client_with_callback(auth: OAuthClient, base_url: String) -> PsaClient {
        let mut client = PsaClient::new(auth, Some(base_url));
        client.set_callback_id("test_cb_id".to_string());
        client
    }

    // [utest->req~vehicle-list~1]
    #[tokio::test]
    async fn test_get_vehicles() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/user/vehicles"))
            .and(header("Authorization", "Bearer test_access_token"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "_embedded": {
                    "vehicles": [
                        {
                            "id": "vehicle123",
                            "vin": "VF3XXXXXXXXXXXXX",
                            "brand": "Peugeot",
                            "label": "e-208",
                            "motorization": "Electric"
                        }
                    ]
                },
                "total": 1,
                "currentPage": 0,
                "totalPage": 1
            })))
            .mount(&mock_server)
            .await;

        let auth = mock_auth();
        let mut client = PsaClient::new(auth, Some(mock_server.uri()));

        let vehicles = client.get_vehicles().await.unwrap();
        assert_eq!(vehicles.len(), 1);
        assert_eq!(vehicles[0].vin, "VF3XXXXXXXXXXXXX");
        assert_eq!(vehicles[0].brand.as_deref(), Some("Peugeot"));
    }

    // [utest->req~vehicle-model-completeness~1]
    #[tokio::test]
    async fn test_vehicle_model_includes_motorization() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/user/vehicles"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "_embedded": {
                    "vehicles": [{
                        "id": "v1",
                        "vin": "VF3XXXXXXXXXXXXX",
                        "motorization": "Electric",
                        "createdAt": "2025-01-01T00:00:00Z",
                        "updatedAt": "2026-04-01T12:00:00Z"
                    }]
                },
                "total": 1,
                "currentPage": 0,
                "totalPage": 1
            })))
            .mount(&mock_server)
            .await;

        let auth = mock_auth();
        let mut client = PsaClient::new(auth, Some(mock_server.uri()));
        let vehicles = client.get_vehicles().await.unwrap();
        assert_eq!(vehicles[0].motorization.as_deref(), Some("Electric"));
        assert!(vehicles[0].created_at.is_some());
        assert!(vehicles[0].updated_at.is_some());
    }

    // [utest->req~vehicle-status~1]
    #[tokio::test]
    async fn test_get_vehicle_status() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/user/vehicles/vehicle123/status"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "updatedAt": "2026-01-15T10:30:00Z",
                "energy": [{
                    "type": "Electric",
                    "level": 75.0,
                    "autonomy": 220.0,
                    "charging": {
                        "status": "Disconnected",
                        "chargingMode": "No"
                    }
                }],
                "odometer": { "mileage": 15230.5 },
                "lastPosition": {
                    "type": "Feature",
                    "geometry": {
                        "type": "Point",
                        "coordinates": [2.3522, 48.8566]
                    }
                }
            })))
            .mount(&mock_server)
            .await;

        let auth = mock_auth();
        let mut client = PsaClient::new(auth, Some(mock_server.uri()));

        let status = client.get_vehicle_status("vehicle123").await.unwrap();
        assert!(status.energy.is_some());
        let energy = &status.energy.unwrap()[0];
        assert_eq!(energy.level, Some(75.0));
        assert_eq!(energy.autonomy, Some(220.0));
        assert_eq!(status.odometer.unwrap().mileage, Some(15230.5));
    }

    // [utest->req~callback-registration~1]
    #[tokio::test]
    async fn test_register_callback() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/user/callbacks"))
            .and(header("Content-Type", "application/json"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "callbackId": "cb_123",
                "status": "Running"
            })))
            .mount(&mock_server)
            .await;

        let auth = mock_auth();
        let mut client = PsaClient::new(auth, Some(mock_server.uri()));
        let id = client.register_callback().await.unwrap();
        assert_eq!(id, "cb_123");
    }

    // [utest->req~remote-command-schema~1]
    // [utest->req~vehicle-wakeup~1]
    #[tokio::test]
    async fn test_wakeup() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path(
                "/user/vehicles/vehicle123/callbacks/test_cb_id/remotes",
            ))
            .and(header("Content-Type", "application/json"))
            .respond_with(ResponseTemplate::new(202).set_body_json(serde_json::json!({
                "remoteActionId": "ra_1",
                "type": "WakeUp"
            })))
            .mount(&mock_server)
            .await;

        let auth = mock_auth();
        let mut client = mock_client_with_callback(auth, mock_server.uri());
        client.wakeup("vehicle123").await.unwrap();
    }

    // [utest->req~charge-control~1]
    // [utest->req~remote-command-schema~1]
    #[tokio::test]
    async fn test_start_charge() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/user/vehicles/v1/callbacks/test_cb_id/remotes"))
            .and(header("Content-Type", "application/json"))
            .respond_with(ResponseTemplate::new(202).set_body_json(serde_json::json!({
                "remoteActionId": "ra_2",
                "type": "ElectricBatteryChargingRequest"
            })))
            .mount(&mock_server)
            .await;

        let auth = mock_auth();
        let mut client = mock_client_with_callback(auth, mock_server.uri());
        client.set_charge("v1", true).await.unwrap();
    }

    // [utest->req~charge-threshold~1]
    // [utest->req~remote-command-schema~1]
    #[tokio::test]
    async fn test_set_charge_threshold() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/user/vehicles/v1/callbacks/test_cb_id/remotes"))
            .respond_with(ResponseTemplate::new(202).set_body_json(serde_json::json!({
                "remoteActionId": "ra_3",
                "type": "ElectricBatteryChargingRequest"
            })))
            .mount(&mock_server)
            .await;

        let auth = mock_auth();
        let mut client = mock_client_with_callback(auth, mock_server.uri());
        client.set_charge_threshold("v1", 80).await.unwrap();
    }

    // [utest->req~charge-scheduling~1]
    // [utest->req~remote-command-schema~1]
    #[tokio::test]
    async fn test_set_charge_schedule() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/user/vehicles/v1/callbacks/test_cb_id/remotes"))
            .respond_with(ResponseTemplate::new(202).set_body_json(serde_json::json!({
                "remoteActionId": "ra_4",
                "type": "ElectricBatteryChargingRequest"
            })))
            .mount(&mock_server)
            .await;

        let auth = mock_auth();
        let mut client = mock_client_with_callback(auth, mock_server.uri());
        client.set_charge_schedule("v1", 6, 0).await.unwrap();
    }

    // [utest->req~preconditioning-control~1]
    // [utest->req~remote-command-schema~1]
    #[tokio::test]
    async fn test_preconditioning() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/user/vehicles/v1/callbacks/test_cb_id/remotes"))
            .respond_with(ResponseTemplate::new(202).set_body_json(serde_json::json!({
                "remoteActionId": "ra_5",
                "type": "ThermalPreconditioning"
            })))
            .mount(&mock_server)
            .await;

        let auth = mock_auth();
        let mut client = mock_client_with_callback(auth, mock_server.uri());
        client.set_preconditioning("v1", true).await.unwrap();
    }

    // [utest->req~door-lock-control~1]
    // [utest->req~remote-command-schema~1]
    #[tokio::test]
    async fn test_door_lock() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/user/vehicles/v1/callbacks/test_cb_id/remotes"))
            .respond_with(ResponseTemplate::new(202).set_body_json(serde_json::json!({
                "remoteActionId": "ra_6",
                "type": "Doors"
            })))
            .mount(&mock_server)
            .await;

        let auth = mock_auth();
        let mut client = mock_client_with_callback(auth, mock_server.uri());
        client.set_door_lock("v1", true).await.unwrap();
    }

    // [utest->req~lights-horn-control~1]
    // [utest->req~remote-command-schema~1]
    #[tokio::test]
    async fn test_flash_lights() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/user/vehicles/v1/callbacks/test_cb_id/remotes"))
            .respond_with(ResponseTemplate::new(202).set_body_json(serde_json::json!({
                "remoteActionId": "ra_7",
                "type": "Lights"
            })))
            .mount(&mock_server)
            .await;

        let auth = mock_auth();
        let mut client = mock_client_with_callback(auth, mock_server.uri());
        client.flash_lights("v1", 10).await.unwrap();
    }

    // [utest->req~lights-horn-control~1]
    // [utest->req~remote-command-schema~1]
    #[tokio::test]
    async fn test_honk_horn() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/user/vehicles/v1/callbacks/test_cb_id/remotes"))
            .respond_with(ResponseTemplate::new(202).set_body_json(serde_json::json!({
                "remoteActionId": "ra_8",
                "type": "Horn"
            })))
            .mount(&mock_server)
            .await;

        let auth = mock_auth();
        let mut client = mock_client_with_callback(auth, mock_server.uri());
        client.honk_horn("v1", 3).await.unwrap();
    }

    // [utest->req~api-error-parsing~1]
    #[tokio::test]
    async fn test_structured_error_parsing() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/user/vehicles"))
            .respond_with(ResponseTemplate::new(404).set_body_json(serde_json::json!({
                "code": 40499,
                "uuid": "494f61d1-472a-4696-ac3c-2961496c3aaf",
                "message": "No data available for such context.",
                "timestamp": "2026-01-01T00:00:00.000Z"
            })))
            .mount(&mock_server)
            .await;

        let auth = mock_auth();
        let mut client = PsaClient::new(auth, Some(mock_server.uri()));
        let err = client.get_vehicles().await.unwrap_err();
        match err {
            PsaError::Api {
                status,
                ref structured,
                ..
            } => {
                assert_eq!(status, 404);
                let s = structured.as_ref().unwrap();
                assert_eq!(s.code, 40499);
                assert_eq!(s.uuid, "494f61d1-472a-4696-ac3c-2961496c3aaf");
            }
            _ => panic!("Expected Api error, got: {:?}", err),
        }
    }

    // [utest->req~rate-limit-handling~1]
    #[tokio::test]
    async fn test_rate_limit_429() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/user/vehicles"))
            .respond_with(
                ResponseTemplate::new(429)
                    .insert_header("Retry-After", "120")
                    .insert_header("X-RateLimit-Remaining-1", "0"),
            )
            .mount(&mock_server)
            .await;

        let auth = mock_auth();
        let mut client = PsaClient::new(auth, Some(mock_server.uri()));
        let err = client.get_vehicles().await.unwrap_err();
        match err {
            PsaError::RateLimited { retry_after_secs } => {
                assert_eq!(retry_after_secs, 120);
            }
            _ => panic!("Expected RateLimited error, got: {:?}", err),
        }
    }

    // [utest->req~api-pagination~1]
    #[test]
    fn test_extract_page_token() {
        let url = "https://api.example.com/user/vehicles?pageSize=60&pageToken=abc123";
        assert_eq!(extract_page_token(url), Some("abc123".to_string()));

        let url_no_token = "https://api.example.com/user/vehicles?pageSize=60";
        assert_eq!(extract_page_token(url_no_token), None);
    }

    // [utest->req~api-pagination~1]
    #[tokio::test]
    async fn test_pagination_follows_next_link() {
        let mock_server = MockServer::start().await;

        // Page 1
        Mock::given(method("GET"))
            .and(path("/user/vehicles"))
            .and(wiremock::matchers::query_param("pageSize", "60"))
            .and(wiremock::matchers::query_param_is_missing("pageToken"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "_embedded": {
                    "vehicles": [{"id": "v1", "vin": "VIN1"}]
                },
                "_links": {
                    "next": {"href": "/user/vehicles?pageSize=60&pageToken=page2tok"}
                },
                "total": 2,
                "currentPage": 0,
                "totalPage": 2
            })))
            .expect(1)
            .mount(&mock_server)
            .await;

        // Page 2
        Mock::given(method("GET"))
            .and(path("/user/vehicles"))
            .and(wiremock::matchers::query_param("pageToken", "page2tok"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "_embedded": {
                    "vehicles": [{"id": "v2", "vin": "VIN2"}]
                },
                "total": 2,
                "currentPage": 1,
                "totalPage": 2
            })))
            .expect(1)
            .mount(&mock_server)
            .await;

        let auth = mock_auth();
        let mut client = PsaClient::new(auth, Some(mock_server.uri()));
        let vehicles = client.get_vehicles().await.unwrap();
        assert_eq!(vehicles.len(), 2);
        assert_eq!(vehicles[0].id, "v1");
        assert_eq!(vehicles[1].id, "v2");
    }

    // [utest->req~oauth2-scope-management~1]
    #[test]
    fn test_default_scopes_include_required_permissions() {
        use crate::auth::DEFAULT_SCOPES;
        assert!(DEFAULT_SCOPES.contains("data:telemetry"));
        assert!(DEFAULT_SCOPES.contains("data:position"));
        assert!(DEFAULT_SCOPES.contains("remote:door:write"));
        assert!(DEFAULT_SCOPES.contains("remote:charging:write"));
        assert!(DEFAULT_SCOPES.contains("remote:wakeup:write"));
    }
}
