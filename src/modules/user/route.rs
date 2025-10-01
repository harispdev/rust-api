use axum::{
    routing::{get, post},
    Router,
};

use crate::common::AppState;

use super::controller::*;

/// Create user routes
pub fn create_routes() -> Router<AppState> {
    Router::new()
        .route("/users", get(get_all).post(create))
        .route("/users/:id", get(get_by_id).put(update).delete(delete_user))
        .route("/users/:id/deactivate", post(deactivate_user))
        .route("/users/:id/activate", post(activate_user))
        .route("/users/account/:account_id", get(get_by_account_id))
        .route("/users/branch/:branch_id", get(get_by_branch_id))
        .route("/users/role/:role", get(get_by_role))
}
