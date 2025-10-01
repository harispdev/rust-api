use axum::{
    routing::{delete, post},
    Router,
};

use crate::common::AppState;
use super::controller::*;

/// Create auth routes
pub fn create_routes() -> Router<AppState> {
    Router::new()
        .route("/auth/register", post(register))
        .route("/auth/login", post(login))
        .route("/auth/logout", delete(logout))
}
