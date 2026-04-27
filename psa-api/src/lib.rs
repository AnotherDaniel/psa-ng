// [impl->req~rust-best-practices~1]
#![deny(warnings)]
#![deny(clippy::all)]

//! PSA Connected Car v4 API client library.
//!
//! Provides OAuth2 authentication, vehicle status retrieval, and remote command
//! execution against the Groupe PSA (Stellantis) Connected Car REST API.

pub mod auth;
pub mod client;
pub mod config;
pub mod error;
pub mod models;
