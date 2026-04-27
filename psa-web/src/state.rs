//! Shared application state injected into axum handlers via [`Arc`].

use psa_api::client::PsaClient;
use psa_api::config::AppConfig;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::db::Database;

/// Shared application state for all axum request handlers.
pub struct AppState {
    /// PSA API client, behind an async mutex for exclusive access during requests.
    pub psa_client: Mutex<PsaClient>,
    /// Application configuration, mutable for runtime settings changes.
    pub config: Mutex<AppConfig>,
    /// Path to the TOML configuration file on disk.
    pub config_path: PathBuf,
    /// SQLite database handle (thread-safe via internal `std::sync::Mutex`).
    pub db: Arc<Database>,
}
