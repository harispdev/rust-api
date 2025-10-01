use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

/// Custom error types for the API
#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("User not found")]
    UserNotFound,
    
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            ApiError::UserNotFound => (StatusCode::NOT_FOUND, "User not found".to_string()),
            ApiError::InvalidInput(msg) => (StatusCode::BAD_REQUEST, msg),
        };

        let body = Json(json!({
            "success": false,
            "error": error_message,
            "timestamp": chrono::Utc::now()
        }));

        (status, body).into_response()
    }
}
