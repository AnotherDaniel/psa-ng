//! HTTP route definitions, middleware, and request handlers.

use axum::{
    Router,
    extract::{Path, Query, Request, State},
    http::StatusCode,
    middleware::{self, Next},
    response::{Html, IntoResponse, Json, Response},
    routing::{get, post},
};
use psa_api::models::VehicleOverview;
use serde::Deserialize;
use std::sync::Arc;

use crate::state::AppState;
use crate::templates;

// [impl->req~request-body-limit~1]
const MAX_BODY_SIZE: usize = 64 * 1024; // 64 KB

/// Build the complete axum router with API and page routes.
pub fn create_router(state: Arc<AppState>) -> Router {
    // API routes requiring bearer token authentication
    let api_routes = Router::new()
        // [impl->req~vehicle-status-endpoint~1]
        .route("/api/vehicles", get(api_get_vehicles))
        .route("/api/vehicles/{id}/status", get(api_get_vehicle_status))
        // [impl->req~wakeup-endpoint~1]
        .route("/api/vehicles/{id}/wakeup", post(api_wakeup))
        // [impl->req~charge-control-endpoint~1]
        .route("/api/vehicles/{id}/charge", post(api_charge))
        .route(
            "/api/vehicles/{id}/charge/threshold",
            post(api_charge_threshold),
        )
        .route(
            "/api/vehicles/{id}/charge/schedule",
            post(api_charge_schedule),
        )
        // [impl->req~preconditioning-endpoint~1]
        .route(
            "/api/vehicles/{id}/preconditioning",
            post(api_preconditioning),
        )
        // [impl->req~door-lock-endpoint~1]
        .route("/api/vehicles/{id}/doors", post(api_door_lock))
        // [impl->req~lights-horn-endpoint~1]
        .route("/api/vehicles/{id}/lights", post(api_lights))
        .route("/api/vehicles/{id}/horn", post(api_horn))
        // [impl->req~settings-endpoint~1]
        .route("/api/settings", get(api_get_settings))
        .route("/api/settings", post(api_update_settings))
        // [impl->req~trips-endpoint~1]
        .route("/api/trips", get(api_get_trips))
        // [impl->req~charging-sessions-endpoint~1]
        .route("/api/charging-sessions", get(api_get_charging_sessions))
        // [impl->req~api-bearer-auth~1]
        .layer(middleware::from_fn_with_state(
            state.clone(),
            api_auth_middleware,
        ));

    // Dashboard pages (no API auth — browser-facing)
    let page_routes = Router::new()
        .route("/", get(dashboard_page))
        .route("/charge", get(charge_page))
        .route("/trips", get(trips_page))
        .route("/settings", get(settings_page));

    page_routes
        .merge(api_routes)
        // [impl->req~security-headers~1]
        .layer(middleware::from_fn(security_headers_middleware))
        .layer(axum::extract::DefaultBodyLimit::max(MAX_BODY_SIZE))
        .with_state(state)
}

// [impl->req~api-bearer-auth~1]
/// Middleware that validates the `Authorization: Bearer <token>` header against the configured token.
async fn api_auth_middleware(
    State(state): State<Arc<AppState>>,
    request: Request,
    next: Next,
) -> Response {
    let config = state.config.lock().await;
    if let Some(ref expected_token) = config.server.api_token {
        let auth_header = request
            .headers()
            .get("Authorization")
            .and_then(|v| v.to_str().ok());

        let provided_token = auth_header.and_then(|h| h.strip_prefix("Bearer "));

        if provided_token != Some(expected_token.as_str()) {
            drop(config);
            return (
                StatusCode::UNAUTHORIZED,
                Json(serde_json::json!({"error": "Invalid or missing bearer token"})),
            )
                .into_response();
        }
    }
    drop(config);
    next.run(request).await
}

// [impl->req~security-headers~1]
/// Middleware that injects security-related HTTP response headers.
async fn security_headers_middleware(request: Request, next: Next) -> Response {
    let mut response = next.run(request).await;
    let headers = response.headers_mut();
    headers.insert("X-Content-Type-Options", "nosniff".parse().unwrap());
    headers.insert("X-Frame-Options", "DENY".parse().unwrap());
    headers.insert(
        "Referrer-Policy",
        "strict-origin-when-cross-origin".parse().unwrap(),
    );
    headers.insert(
        "Content-Security-Policy",
        "default-src 'self'; script-src 'self' 'unsafe-inline'; style-src 'self' 'unsafe-inline'"
            .parse()
            .unwrap(),
    );
    response
}

// [impl->req~sanitized-errors~1]
/// Strip potentially sensitive details (paths, URLs) from error messages before returning to clients.
fn sanitize_error(e: &dyn std::fmt::Display) -> String {
    let msg = e.to_string();
    // Strip internal details: file paths, URLs, token contents
    if msg.contains("http://") || msg.contains("https://") || msg.contains('/') {
        "An internal error occurred".to_string()
    } else {
        msg
    }
}

// ── Dashboard pages ──────────────────────────────────────────────────

// [impl->req~dashboard-overview~1]
/// Render the main dashboard showing an overview of all vehicles.
async fn dashboard_page(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let mut client = state.psa_client.lock().await;

    let overviews: Vec<VehicleOverview> = if client.has_authentication() {
        match client.get_vehicles().await {
            Ok(vehicles) => {
                let mut ovs = Vec::new();
                for v in &vehicles {
                    if let Ok(status) = client.get_vehicle_status(&v.id).await {
                        ovs.push(VehicleOverview::from_status(v, &status));
                    }
                }
                ovs
            }
            Err(_) => Vec::new(),
        }
    } else {
        Vec::new()
    };

    let html = templates::render_dashboard(&overviews);
    Html(html)
}

// [impl->req~charge-management-page~1]
/// Render the charge management page.
async fn charge_page(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let client = state.psa_client.lock().await;
    let authenticated = client.has_authentication();
    let html = templates::render_charge_page(authenticated);
    Html(html)
}

// [impl->req~trip-display-page~1]
/// Render the trips history page.
async fn trips_page(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let trips = state.db.get_trips(None).unwrap_or_default();
    let html = templates::render_trips_page(&trips);
    Html(html)
}

// [impl->req~settings-page~1]
/// Render the electricity pricing settings page.
async fn settings_page(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let config = state.config.lock().await;
    let html = templates::render_settings_page(&config);
    Html(html)
}

// ── REST API endpoints ───────────────────────────────────────────────

async fn api_get_vehicles(
    State(state): State<Arc<AppState>>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let mut client = state.psa_client.lock().await;
    match client.get_vehicles().await {
        Ok(vehicles) => Ok(Json(serde_json::to_value(vehicles).unwrap())),
        Err(e) => Err((StatusCode::BAD_GATEWAY, sanitize_error(&e))),
    }
}

async fn api_get_vehicle_status(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let mut client = state.psa_client.lock().await;
    match client.get_vehicle_status(&id).await {
        Ok(status) => Ok(Json(serde_json::to_value(status).unwrap())),
        Err(e) => Err((StatusCode::BAD_GATEWAY, sanitize_error(&e))),
    }
}

async fn api_wakeup(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let mut client = state.psa_client.lock().await;
    match client.wakeup(&id).await {
        Ok(()) => Ok(Json(serde_json::json!({"status": "ok"}))),
        Err(e) => Err((StatusCode::BAD_GATEWAY, sanitize_error(&e))),
    }
}

/// Start or stop charging.
#[derive(Deserialize)]
struct ChargeParams {
    start: bool,
}

async fn api_charge(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Json(params): Json<ChargeParams>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let mut client = state.psa_client.lock().await;
    match client.set_charge(&id, params.start).await {
        Ok(()) => Ok(Json(serde_json::json!({"status": "ok"}))),
        Err(e) => Err((StatusCode::BAD_GATEWAY, sanitize_error(&e))),
    }
}

/// Set the charging threshold percentage.
#[derive(Deserialize)]
struct ThresholdParams {
    percentage: u8,
}

async fn api_charge_threshold(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Json(params): Json<ThresholdParams>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let mut client = state.psa_client.lock().await;
    match client.set_charge_threshold(&id, params.percentage).await {
        Ok(()) => Ok(Json(serde_json::json!({"status": "ok"}))),
        Err(e) => Err((StatusCode::BAD_GATEWAY, sanitize_error(&e))),
    }
}

/// Set a charging schedule (time of day).
#[derive(Deserialize)]
struct ScheduleParams {
    hour: u8,
    minute: u8,
}

async fn api_charge_schedule(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Json(params): Json<ScheduleParams>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let mut client = state.psa_client.lock().await;
    match client
        .set_charge_schedule(&id, params.hour, params.minute)
        .await
    {
        Ok(()) => Ok(Json(serde_json::json!({"status": "ok"}))),
        Err(e) => Err((StatusCode::BAD_GATEWAY, sanitize_error(&e))),
    }
}

/// Start or stop cabin preconditioning.
#[derive(Deserialize)]
struct PreconditioningParams {
    start: bool,
}

async fn api_preconditioning(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Json(params): Json<PreconditioningParams>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let mut client = state.psa_client.lock().await;
    match client.set_preconditioning(&id, params.start).await {
        Ok(()) => Ok(Json(serde_json::json!({"status": "ok"}))),
        Err(e) => Err((StatusCode::BAD_GATEWAY, sanitize_error(&e))),
    }
}

/// Lock or unlock vehicle doors.
#[derive(Deserialize)]
struct DoorLockParams {
    lock: bool,
}

async fn api_door_lock(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Json(params): Json<DoorLockParams>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let mut client = state.psa_client.lock().await;
    match client.set_door_lock(&id, params.lock).await {
        Ok(()) => Ok(Json(serde_json::json!({"status": "ok"}))),
        Err(e) => Err((StatusCode::BAD_GATEWAY, sanitize_error(&e))),
    }
}

/// Flash lights for a duration.
#[derive(Deserialize)]
struct LightsParams {
    duration: u32,
}

async fn api_lights(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Json(params): Json<LightsParams>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let mut client = state.psa_client.lock().await;
    match client.flash_lights(&id, params.duration).await {
        Ok(()) => Ok(Json(serde_json::json!({"status": "ok"}))),
        Err(e) => Err((StatusCode::BAD_GATEWAY, sanitize_error(&e))),
    }
}

/// Honk the horn a number of times.
#[derive(Deserialize)]
struct HornParams {
    count: u32,
}

async fn api_horn(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Json(params): Json<HornParams>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let mut client = state.psa_client.lock().await;
    match client.honk_horn(&id, params.count).await {
        Ok(()) => Ok(Json(serde_json::json!({"status": "ok"}))),
        Err(e) => Err((StatusCode::BAD_GATEWAY, sanitize_error(&e))),
    }
}

async fn api_get_settings(State(state): State<Arc<AppState>>) -> Json<serde_json::Value> {
    let config = state.config.lock().await;
    Json(serde_json::to_value(&config.electricity).unwrap())
}

/// Partial update of electricity pricing settings.
#[derive(Deserialize)]
struct SettingsUpdate {
    price_per_kwh: Option<f64>,
    night_price_per_kwh: Option<f64>,
    night_start_hour: Option<u8>,
    night_start_minute: Option<u8>,
    night_end_hour: Option<u8>,
    night_end_minute: Option<u8>,
    currency: Option<String>,
}

async fn api_update_settings(
    State(state): State<Arc<AppState>>,
    Json(update): Json<SettingsUpdate>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let mut config = state.config.lock().await;

    if let Some(v) = update.price_per_kwh {
        config.electricity.price_per_kwh = v;
    }
    if let Some(v) = update.night_price_per_kwh {
        config.electricity.night_price_per_kwh = Some(v);
    }
    if let Some(v) = update.night_start_hour {
        config.electricity.night_start_hour = Some(v);
    }
    if let Some(v) = update.night_start_minute {
        config.electricity.night_start_minute = Some(v);
    }
    if let Some(v) = update.night_end_hour {
        config.electricity.night_end_hour = Some(v);
    }
    if let Some(v) = update.night_end_minute {
        config.electricity.night_end_minute = Some(v);
    }
    if let Some(v) = update.currency {
        config.electricity.currency = v;
    }

    config
        .save(&state.config_path)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, sanitize_error(&e)))?;

    Ok(Json(serde_json::json!({"status": "ok"})))
}

/// Optional VIN filter for trip queries.
#[derive(Deserialize)]
struct TripQuery {
    vin: Option<String>,
}

async fn api_get_trips(
    State(state): State<Arc<AppState>>,
    Query(query): Query<TripQuery>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let trips = state
        .db
        .get_trips(query.vin.as_deref())
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, sanitize_error(&e)))?;
    Ok(Json(serde_json::to_value(trips).unwrap()))
}

/// Optional VIN filter for charging session queries.
#[derive(Deserialize)]
struct ChargingQuery {
    vin: Option<String>,
}

async fn api_get_charging_sessions(
    State(state): State<Arc<AppState>>,
    Query(query): Query<ChargingQuery>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let sessions = state
        .db
        .get_charging_sessions(query.vin.as_deref())
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, sanitize_error(&e)))?;
    Ok(Json(serde_json::to_value(sessions).unwrap()))
}
