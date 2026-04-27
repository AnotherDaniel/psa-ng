//! Data models for the PSA Connected Car v4 API responses.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

// [impl->req~vehicle-list~1]
/// A vehicle registered to the authenticated user.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vehicle {
    pub id: String,
    pub vin: String,
    pub brand: Option<String>,
    pub label: Option<String>,
}

/// Wrapper for the paginated vehicles list response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VehiclesResponse {
    #[serde(rename = "_embedded")]
    pub embedded: Option<VehiclesEmbedded>,
}

/// Embedded container for the vehicles array.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VehiclesEmbedded {
    pub vehicles: Vec<Vehicle>,
}

// [impl->req~vehicle-status~1]
/// Full vehicle status as returned by the PSA API.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VehicleStatus {
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<DateTime<Utc>>,
    pub battery: Option<Battery>,
    pub environment: Option<Environment>,
    pub odometer: Option<Odometer>,
    #[serde(rename = "lastPosition")]
    pub last_position: Option<Position>,
    pub preconditionning: Option<Preconditioning>,
    #[serde(rename = "doorsState")]
    pub doors_state: Option<DoorsState>,
    pub energy: Option<Vec<Energy>>,
    pub kinetic: Option<Kinetic>,
    pub safety: Option<Safety>,
}

/// Vehicle battery voltage and current.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Battery {
    pub voltage: Option<f64>,
    pub current: Option<f64>,
}

/// Environmental sensor data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Environment {
    #[serde(rename = "air")]
    pub air: Option<AirEnvironment>,
}

/// Air temperature reading.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AirEnvironment {
    pub temp: Option<f64>,
}

/// Odometer (mileage) reading.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Odometer {
    pub mileage: Option<f64>,
}

/// GeoJSON-style vehicle position.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    #[serde(rename = "type")]
    pub position_type: Option<String>,
    pub geometry: Option<Geometry>,
    pub properties: Option<PositionProperties>,
}

/// GeoJSON geometry with coordinates.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Geometry {
    #[serde(rename = "type")]
    pub geometry_type: Option<String>,
    pub coordinates: Option<Vec<f64>>,
}

/// Position metadata (heading, timestamp).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PositionProperties {
    pub heading: Option<f64>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<DateTime<Utc>>,
}

/// Cabin preconditioning state.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Preconditioning {
    #[serde(rename = "airConditioning")]
    pub air_conditioning: Option<AirConditioning>,
}

/// Air conditioning status.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AirConditioning {
    pub status: Option<String>,
}

/// Door opening states and lock status.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DoorsState {
    pub opening: Option<Vec<DoorOpening>>,
    pub locked: Option<Vec<String>>,
}

/// Individual door opening state.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DoorOpening {
    pub identifier: Option<String>,
    pub state: Option<String>,
}

/// Energy source (battery level, charging status, autonomy).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Energy {
    #[serde(rename = "type")]
    pub energy_type: Option<String>,
    pub level: Option<f64>,
    pub charging: Option<Charging>,
    pub autonomy: Option<f64>,
}

/// Charging state and progress.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Charging {
    pub status: Option<String>,
    #[serde(rename = "chargingMode")]
    pub charging_mode: Option<String>,
    #[serde(rename = "chargingRate")]
    pub charging_rate: Option<f64>,
    #[serde(rename = "remainingTime")]
    pub remaining_time: Option<String>,
    #[serde(rename = "nextDelayedTime")]
    pub next_delayed_time: Option<String>,
}

/// Vehicle motion state.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Kinetic {
    pub moving: Option<bool>,
    pub speed: Option<f64>,
}

/// Safety system indicators.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Safety {
    #[serde(rename = "beltWarning")]
    pub belt_warning: Option<String>,
    #[serde(rename = "eCallTriggeringRequest")]
    pub ecall_triggering_request: Option<String>,
}

/// Simplified vehicle overview for dashboard display
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VehicleOverview {
    pub vin: String,
    pub brand: Option<String>,
    pub label: Option<String>,
    pub battery_level: Option<f64>,
    pub charging_status: Option<String>,
    pub mileage_km: Option<f64>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub last_updated: Option<DateTime<Utc>>,
    pub autonomy_km: Option<f64>,
}

impl VehicleOverview {
    /// Construct a dashboard-friendly overview from raw API data.
    pub fn from_status(vehicle: &Vehicle, status: &VehicleStatus) -> Self {
        let (battery_level, charging_status, autonomy_km) = status
            .energy
            .as_ref()
            .and_then(|energies| {
                energies
                    .iter()
                    .find(|e| e.energy_type.as_deref() == Some("Electric"))
                    .map(|e| {
                        (
                            e.level,
                            e.charging.as_ref().and_then(|c| c.status.clone()),
                            e.autonomy,
                        )
                    })
            })
            .unwrap_or((None, None, None));

        let (latitude, longitude) = status
            .last_position
            .as_ref()
            .and_then(|p| p.geometry.as_ref())
            .and_then(|g| g.coordinates.as_ref())
            .map(|coords| {
                let lon = coords.first().copied();
                let lat = coords.get(1).copied();
                (lat, lon)
            })
            .unwrap_or((None, None));

        Self {
            vin: vehicle.vin.clone(),
            brand: vehicle.brand.clone(),
            label: vehicle.label.clone(),
            battery_level,
            charging_status,
            mileage_km: status.odometer.as_ref().and_then(|o| o.mileage),
            latitude,
            longitude,
            last_updated: status.updated_at,
            autonomy_km,
        }
    }
}

/// Trip record for persistence
// [impl->req~trip-recording~1]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Trip {
    pub id: i64,
    pub vin: String,
    pub start_at: DateTime<Utc>,
    pub end_at: DateTime<Utc>,
    pub start_lat: Option<f64>,
    pub start_lon: Option<f64>,
    pub end_lat: Option<f64>,
    pub end_lon: Option<f64>,
    pub distance_km: Option<f64>,
    pub consumption_kwh: Option<f64>,
}

/// Charging session record for persistence
// [impl->req~charging-session-recording~1]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChargingSession {
    pub id: i64,
    pub vin: String,
    pub start_at: DateTime<Utc>,
    pub end_at: Option<DateTime<Utc>>,
    pub start_level: Option<f64>,
    pub end_level: Option<f64>,
    pub energy_kwh: Option<f64>,
    pub cost: Option<f64>,
}
