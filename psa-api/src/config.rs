// [impl->req~configuration-file~1]

//! TOML-based application configuration.

use crate::error::{PsaError, Result};
use serde::{Deserialize, Serialize};

/// Top-level application configuration loaded from a TOML file.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    /// PSA Connected Car API credentials and endpoint settings.
    pub psa: PsaConfig,
    /// HTTP server binding and authentication settings.
    #[serde(default)]
    pub server: ServerConfig,
    /// Electricity pricing for cost calculations.
    #[serde(default)]
    pub electricity: ElectricityConfig,
}

/// PSA Connected Car API credentials.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PsaConfig {
    /// OAuth2 client ID.
    pub client_id: String,
    /// OAuth2 client secret.
    pub client_secret: String,
    /// Vehicle brand (`peugeot`, `citroen`, `ds`, `opel`, `vauxhall`).
    pub brand: String,
    /// Base URL for the Connected Car v4 API.
    #[serde(default = "default_api_base_url")]
    pub api_base_url: String,
    /// Path to the persisted OAuth token file (overrides the default in `data_dir`).
    pub token_file: Option<String>,
}

fn default_api_base_url() -> String {
    "https://api.groupe-psa.com/connectedcar/v4".to_string()
}

/// HTTP server configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    /// Listen address (default `127.0.0.1`).
    #[serde(default = "default_host")]
    pub host: String,
    /// Listen port (default `5000`).
    #[serde(default = "default_port")]
    pub port: u16,
    /// Directory for persistent data (database, tokens).
    pub data_dir: Option<String>,
    /// Optional bearer token required for API endpoint access.
    pub api_token: Option<String>,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            host: default_host(),
            port: default_port(),
            data_dir: None,
            api_token: None,
        }
    }
}

fn default_host() -> String {
    "127.0.0.1".to_string()
}

fn default_port() -> u16 {
    5000
}

// [impl->req~electricity-pricing~1]
/// Electricity pricing configuration for charging cost calculations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElectricityConfig {
    /// Standard price per kilowatt-hour.
    #[serde(default)]
    pub price_per_kwh: f64,
    /// Optional off-peak (night) price per kilowatt-hour.
    pub night_price_per_kwh: Option<f64>,
    /// Hour at which the night rate begins (0–23).
    pub night_start_hour: Option<u8>,
    /// Minute at which the night rate begins (0–59).
    pub night_start_minute: Option<u8>,
    /// Hour at which the night rate ends (0–23).
    pub night_end_hour: Option<u8>,
    /// Minute at which the night rate ends (0–59).
    pub night_end_minute: Option<u8>,
    /// Currency code for display (default `EUR`).
    #[serde(default = "default_currency")]
    pub currency: String,
}

impl Default for ElectricityConfig {
    fn default() -> Self {
        Self {
            price_per_kwh: 0.0,
            night_price_per_kwh: None,
            night_start_hour: None,
            night_start_minute: None,
            night_end_hour: None,
            night_end_minute: None,
            currency: default_currency(),
        }
    }
}

fn default_currency() -> String {
    "EUR".to_string()
}

impl AppConfig {
    /// Load configuration from a TOML file at `path`.
    pub fn load(path: &std::path::Path) -> Result<Self> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| PsaError::Config(format!("Failed to read config: {e}")))?;
        toml::from_str(&content)
            .map_err(|e| PsaError::Config(format!("Failed to parse config: {e}")))
    }

    /// Serialize and write the configuration to a TOML file at `path`.
    pub fn save(&self, path: &std::path::Path) -> Result<()> {
        let content = toml::to_string_pretty(self)
            .map_err(|e| PsaError::Config(format!("Failed to serialize config: {e}")))?;
        std::fs::write(path, content)?;
        Ok(())
    }
}
