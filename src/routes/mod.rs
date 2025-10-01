#[allow(unused_imports)]
use axum::{
    routing::{get, post, put, delete},
    Router,
};
use tower_http::cors::CorsLayer;

use crate::{
    handlers::{health, users},
    state::AppState,
};

/// Create the main application router
pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/", get(health::root))
        .route("/health", get(health::health_check))
        .route("/users", get(users::get_users).post(users::create_user))
        .route("/users/:id", get(users::get_user).put(users::update_user).delete(users::delete_user))
        .layer(CorsLayer::permissive())
        .with_state(state)
}
