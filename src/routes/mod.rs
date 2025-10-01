use axum::{
    routing::get,
    Router, middleware,
};

use crate::common::AppState;
use crate::modules::user::route::create_routes as create_user_routes;
use crate::modules::auth::route::create_routes as create_auth_routes;
use crate::modules::auth::middleware::authenticate;

/// Create the main application router
pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health_check))
        .nest("/", create_auth_routes())
        .nest("/", create_user_routes().layer(middleware::from_fn(authenticate)))
        .with_state(state)
}

/// Health check endpoint
async fn health_check() -> axum::Json<serde_json::Value> {
    axum::Json(serde_json::json!({
        "status": "healthy",
        "version": "0.1.0",
        "uptime": 0
    }))
}