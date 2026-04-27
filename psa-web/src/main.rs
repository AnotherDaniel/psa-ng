// [impl->req~rust-best-practices~1]
#![deny(warnings)]
#![deny(clippy::all)]

//! PSA Connected Car web interface and REST API server.

mod db;
mod routes;
mod state;
mod templates;

#[cfg(test)]
mod config_tests;
#[cfg(test)]
mod route_tests;

// [impl->req~http-server~1]

use psa_api::auth::OAuthClient;
use psa_api::client::PsaClient;
use psa_api::config::AppConfig;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::info;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let config_path = std::env::args()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("config.toml"));

    let config = match AppConfig::load(&config_path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!(
                "Failed to load configuration from {}: {}",
                config_path.display(),
                e
            );
            eprintln!("Please create a config.toml file. Example:");
            eprintln!();
            eprintln!(
                r#"[psa]
client_id = "your_client_id"
client_secret = "your_client_secret"
brand = "peugeot"

[server]
host = "127.0.0.1"
port = 5000

[electricity]
price_per_kwh = 0.15
currency = "EUR""#
            );
            std::process::exit(1);
        }
    };

    let data_dir = config
        .server
        .data_dir
        .as_ref()
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("data"));
    std::fs::create_dir_all(&data_dir).expect("Failed to create data directory");

    let db_path = data_dir.join("psa-ng.db");
    let db = db::Database::open(&db_path).expect("Failed to open database");

    let token_path = config
        .psa
        .token_file
        .as_ref()
        .map(PathBuf::from)
        .unwrap_or_else(|| data_dir.join("token.json"));

    let auth = OAuthClient::new(
        config.psa.client_id.clone(),
        config.psa.client_secret.clone(),
        config.psa.brand.clone(),
        Some(token_path),
    );

    let psa_client = PsaClient::new(auth, Some(config.psa.api_base_url.clone()));

    let state = Arc::new(state::AppState {
        psa_client: Mutex::new(psa_client),
        config: Mutex::new(config.clone()),
        config_path: config_path.clone(),
        db: Arc::new(db),
    });

    let app = routes::create_router(state);

    let addr = format!("{}:{}", config.server.host, config.server.port);
    info!("Starting psa-ng on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("Failed to bind address");

    axum::serve(listener, app).await.expect("Server error");
}
