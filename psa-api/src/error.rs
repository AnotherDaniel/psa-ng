//! Error types for the PSA API client.

use serde::{Deserialize, Serialize};
use thiserror::Error;

// [impl->req~api-error-parsing~1]
/// Structured error response from the PSA Connected Car API.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiErrorResponse {
    /// Enhanced HTTP error code (first 3 digits = HTTP status, last 2 = API-specific).
    pub code: u32,
    /// Unique identifier for this error occurrence (for support requests).
    pub uuid: String,
    /// Human-readable error message.
    pub message: String,
    /// Timestamp when the error occurred.
    pub timestamp: String,
}

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

    // [impl->req~api-error-parsing~1]
    /// The PSA API returned a structured error response.
    #[error("API error ({status}): {detail}")]
    Api {
        /// HTTP status code.
        status: u16,
        /// Human-readable error detail.
        detail: String,
        /// Parsed structured error, if available.
        structured: Option<ApiErrorResponse>,
    },

    // [impl->req~rate-limit-handling~1]
    /// The API returned HTTP 429 — rate limit exceeded.
    #[error("Rate limited — retry after {retry_after_secs}s")]
    RateLimited {
        /// Seconds to wait before retrying.
        retry_after_secs: u64,
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
