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
    
    #[error("User already exists")]
    UserAlreadyExists,
    
    #[error("Database error: {0}")]
    DatabaseError(String),
    
    #[error("Unauthorized: {0}")]
    Unauthorized(String),
    
    #[error("Invalid credentials")]
    InvalidCredentials,
    
    #[error("Internal server error")]
    InternalServerError,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            ApiError::UserNotFound => (StatusCode::NOT_FOUND, "User not found".to_string()),
            ApiError::InvalidInput(msg) => (StatusCode::BAD_REQUEST, msg),
            ApiError::UserAlreadyExists => (StatusCode::CONFLICT, "User already exists".to_string()),
            ApiError::DatabaseError(msg) => {
                tracing::error!("Database error: {}", msg);
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error".to_string())
            }
            ApiError::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, msg),
            ApiError::InvalidCredentials => (StatusCode::UNAUTHORIZED, "Invalid credentials".to_string()),
            ApiError::InternalServerError => (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error".to_string()),
        };

        let body = Json(json!({
            "success": false,
            "error": error_message,
            "timestamp": chrono::Utc::now()
        }));

        (status, body).into_response()
    }
}
