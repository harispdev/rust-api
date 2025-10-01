use axum::{extract::State, Json};
use crate::{
    models::HealthResponse,
    state::AppState,
};

/// Health check endpoint
pub async fn health_check(State(state): State<AppState>) -> Json<HealthResponse> {
    let health = HealthResponse::new(state.uptime());
    Json(health)
}

/// Root endpoint with welcome message
pub async fn root() -> Json<&'static str> {
    Json("Welcome to Rust API! 🦀")
}