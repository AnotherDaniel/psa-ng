//! Error types for the PSA API client.

use thiserror::Error;

/// Errors that can occur when interacting with the PSA Connected Car API.
#[derive(Error, Debug)]
pub enum PsaError {
    /// An HTTP transport error from the underlying `reqwest` client.
    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),

    /// JSON serialization or deserialization failed.
    #[error("JSON serialization/deserialization failed: {0}")]
    Json(#[from] serde_json::Error),

    /// Authentication-level failure (invalid credentials, rejected code exchange, etc.).
    #[error("Authentication failed: {0}")]
    Auth(String),

    /// The stored token has expired and could not be refreshed.
    #[error("Token expired and refresh failed")]
    TokenExpired,

    /// The requested vehicle ID was not found.
    #[error("Vehicle not found: {0}")]
    VehicleNotFound(String),

    /// The PSA API returned a non-success HTTP status.
    #[error("API error ({status}): {message}")]
    Api {
        /// HTTP status code.
        status: u16,
        /// Response body or error message.
        message: String,
    },

    /// A configuration file could not be read, parsed, or written.
    #[error("Configuration error: {0}")]
    Config(String),

    /// A filesystem I/O error.
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

/// Convenience alias used throughout the crate.
pub type Result<T> = std::result::Result<T, PsaError>;
