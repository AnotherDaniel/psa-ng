use crate::auth::OAuthClient;
use crate::error::{PsaError, Result};
use crate::models::{Vehicle, VehicleStatus, VehiclesResponse};
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
        }
    }

    /// Build the `Authorization: Bearer <token>` header value.
    async fn auth_header(&mut self) -> Result<String> {
        let token = self.auth.get_valid_token().await?;
        Ok(format!("Bearer {token}"))
    }

    /// Perform an authenticated GET request against the API.
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

        if !response.status().is_success() {
            let status = response.status().as_u16();
            let body = response.text().await.unwrap_or_default();
            return Err(PsaError::Api {
                status,
                message: body,
            });
        }

        Ok(response)
    }

    /// POST a remote command (callback) to the API.
    async fn post_remote_command(&mut self, path: &str, body: &serde_json::Value) -> Result<()> {
        let url = format!("{}{}", self.base_url, path);
        let auth = self.auth_header().await?;

        debug!("POST {}", url);
        let response = self
            .http
            .post(&url)
            .header(AUTHORIZATION, auth)
            .header(CONTENT_TYPE, "application/vnd.api+json")
            .header(ACCEPT, "application/hal+json")
            .json(body)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status().as_u16();
            let body = response.text().await.unwrap_or_default();
            return Err(PsaError::Api {
                status,
                message: body,
            });
        }

        Ok(())
    }

    // [impl->req~vehicle-list~1]
    /// Retrieve all vehicles for the authenticated user
    pub async fn get_vehicles(&mut self) -> Result<Vec<Vehicle>> {
        info!("Fetching vehicle list");
        let response = self.get("/user/vehicles").await?;
        let data: VehiclesResponse = response.json().await?;
        Ok(data.embedded.map(|e| e.vehicles).unwrap_or_default())
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
    /// Send a wakeup request to force the vehicle to report status
    pub async fn wakeup(&mut self, vehicle_id: &str) -> Result<()> {
        info!("Sending wakeup to vehicle {}", vehicle_id);
        self.post_remote_command(
            &format!("/user/vehicles/{vehicle_id}/callbacks"),
            &serde_json::json!({
                "type": "TelemetryRequest"
            }),
        )
        .await
    }

    // [impl->req~charge-control~1]
    /// Start or stop vehicle charging
    pub async fn set_charge(&mut self, vehicle_id: &str, start: bool) -> Result<()> {
        let action = if start { "start" } else { "stop" };
        info!("{}ing charge for vehicle {}", action, vehicle_id);
        self.post_remote_command(
            &format!("/user/vehicles/{vehicle_id}/callbacks"),
            &serde_json::json!({
                "type": "ChargingRequest",
                "action": action
            }),
        )
        .await
    }

    // [impl->req~charge-threshold~1]
    /// Set the charge threshold percentage
    pub async fn set_charge_threshold(&mut self, vehicle_id: &str, percentage: u8) -> Result<()> {
        info!(
            "Setting charge threshold to {}% for vehicle {}",
            percentage, vehicle_id
        );
        self.post_remote_command(
            &format!("/user/vehicles/{vehicle_id}/callbacks"),
            &serde_json::json!({
                "type": "ChargingRequest",
                "programs": [{
                    "type": "immediate",
                    "limitSoc": percentage
                }]
            }),
        )
        .await
    }

    // [impl->req~charge-scheduling~1]
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
        self.post_remote_command(
            &format!("/user/vehicles/{vehicle_id}/callbacks"),
            &serde_json::json!({
                "type": "ChargingRequest",
                "programs": [{
                    "type": "delayed",
                    "dayNight": true,
                    "start": format!("PT{hour}H{minute}M")
                }]
            }),
        )
        .await
    }

    // [impl->req~preconditioning-control~1]
    /// Start or stop air conditioning preconditioning
    pub async fn set_preconditioning(&mut self, vehicle_id: &str, start: bool) -> Result<()> {
        let action = if start { "activate" } else { "deactivate" };
        info!("{}ing preconditioning for vehicle {}", action, vehicle_id);
        self.post_remote_command(
            &format!("/user/vehicles/{vehicle_id}/callbacks"),
            &serde_json::json!({
                "type": "ThermalPreconditioningRequest",
                "action": action
            }),
        )
        .await
    }

    // [impl->req~door-lock-control~1]
    /// Lock or unlock vehicle doors
    pub async fn set_door_lock(&mut self, vehicle_id: &str, lock: bool) -> Result<()> {
        let action = if lock { "lock" } else { "unlock" };
        info!("{}ing doors for vehicle {}", action, vehicle_id);
        self.post_remote_command(
            &format!("/user/vehicles/{vehicle_id}/callbacks"),
            &serde_json::json!({
                "type": "DoorLockRequest",
                "action": action
            }),
        )
        .await
    }

    // [impl->req~lights-horn-control~1]
    /// Flash lights for a given duration (seconds)
    pub async fn flash_lights(&mut self, vehicle_id: &str, duration: u32) -> Result<()> {
        info!(
            "Flashing lights for {}s on vehicle {}",
            duration, vehicle_id
        );
        self.post_remote_command(
            &format!("/user/vehicles/{vehicle_id}/callbacks"),
            &serde_json::json!({
                "type": "LightsRequest",
                "duration": duration
            }),
        )
        .await
    }

    // [impl->req~lights-horn-control~1]
    /// Honk the horn a given number of times
    pub async fn honk_horn(&mut self, vehicle_id: &str, count: u32) -> Result<()> {
        info!("Honking horn {}x on vehicle {}", count, vehicle_id);
        self.post_remote_command(
            &format!("/user/vehicles/{vehicle_id}/callbacks"),
            &serde_json::json!({
                "type": "HornRequest",
                "count": count
            }),
        )
        .await
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
                            "label": "e-208"
                        }
                    ]
                }
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

    // [utest->req~vehicle-wakeup~1]
    #[tokio::test]
    async fn test_wakeup() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/user/vehicles/vehicle123/callbacks"))
            .respond_with(ResponseTemplate::new(200))
            .mount(&mock_server)
            .await;

        let auth = mock_auth();
        let mut client = PsaClient::new(auth, Some(mock_server.uri()));
        client.wakeup("vehicle123").await.unwrap();
    }

    // [utest->req~charge-control~1]
    #[tokio::test]
    async fn test_start_charge() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/user/vehicles/v1/callbacks"))
            .respond_with(ResponseTemplate::new(200))
            .mount(&mock_server)
            .await;

        let auth = mock_auth();
        let mut client = PsaClient::new(auth, Some(mock_server.uri()));
        client.set_charge("v1", true).await.unwrap();
    }

    // [utest->req~charge-threshold~1]
    #[tokio::test]
    async fn test_set_charge_threshold() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/user/vehicles/v1/callbacks"))
            .respond_with(ResponseTemplate::new(200))
            .mount(&mock_server)
            .await;

        let auth = mock_auth();
        let mut client = PsaClient::new(auth, Some(mock_server.uri()));
        client.set_charge_threshold("v1", 80).await.unwrap();
    }

    // [utest->req~charge-scheduling~1]
    #[tokio::test]
    async fn test_set_charge_schedule() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/user/vehicles/v1/callbacks"))
            .respond_with(ResponseTemplate::new(200))
            .mount(&mock_server)
            .await;

        let auth = mock_auth();
        let mut client = PsaClient::new(auth, Some(mock_server.uri()));
        client.set_charge_schedule("v1", 6, 0).await.unwrap();
    }

    // [utest->req~preconditioning-control~1]
    #[tokio::test]
    async fn test_preconditioning() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/user/vehicles/v1/callbacks"))
            .respond_with(ResponseTemplate::new(200))
            .mount(&mock_server)
            .await;

        let auth = mock_auth();
        let mut client = PsaClient::new(auth, Some(mock_server.uri()));
        client.set_preconditioning("v1", true).await.unwrap();
    }

    // [utest->req~door-lock-control~1]
    #[tokio::test]
    async fn test_door_lock() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/user/vehicles/v1/callbacks"))
            .respond_with(ResponseTemplate::new(200))
            .mount(&mock_server)
            .await;

        let auth = mock_auth();
        let mut client = PsaClient::new(auth, Some(mock_server.uri()));
        client.set_door_lock("v1", true).await.unwrap();
    }

    // [utest->req~lights-horn-control~1]
    #[tokio::test]
    async fn test_flash_lights() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/user/vehicles/v1/callbacks"))
            .respond_with(ResponseTemplate::new(200))
            .mount(&mock_server)
            .await;

        let auth = mock_auth();
        let mut client = PsaClient::new(auth, Some(mock_server.uri()));
        client.flash_lights("v1", 10).await.unwrap();
    }

    // [utest->req~lights-horn-control~1]
    #[tokio::test]
    async fn test_honk_horn() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/user/vehicles/v1/callbacks"))
            .respond_with(ResponseTemplate::new(200))
            .mount(&mock_server)
            .await;

        let auth = mock_auth();
        let mut client = PsaClient::new(auth, Some(mock_server.uri()));
        client.honk_horn("v1", 3).await.unwrap();
    }
}
