use axum::{
    extract::State,
    http::StatusCode,
    Json,
};
use tower_sessions::Session;
use tracing::info;
use validator::Validate;

use crate::{
    common::ApiError,
    modules::auth::entity::{LoginRequest, UserInfo},
    modules::user::entity::CreateUserRequest,
    common::{AppState, session::SessionManager},
};

/// Login an existing user
pub async fn login(
    State(state): State<AppState>,
    session: Session,
    Json(payload): Json<LoginRequest>,
) -> Result<StatusCode, ApiError> {
    info!("Login request for email: {}", payload.email);
    
    // Validate the request
    payload.validate()
        .map_err(|e| ApiError::InvalidInput(format!("Validation error: {}", e)))?;
    
    let user_info = state.auth_service.login(payload).await?;
    
    // Store user in session (like req.logIn() in Node.js)
    SessionManager::login(&session, user_info).await
        .map_err(|e| ApiError::InternalServerError)?;
    
    info!("User logged in successfully");
    Ok(StatusCode::OK)
}

/// Register a new user
pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<StatusCode, ApiError> {
    info!("Registration request for email: {}", payload.email);
    
    // Validate the request
    payload.validate()
        .map_err(|e| ApiError::InvalidInput(format!("Validation error: {}", e)))?;
    
    // Create the user
    state.user_service.create(payload).await?;
    
    info!("User registered successfully");
    Ok(StatusCode::CREATED)
}

/// Logout user
pub async fn logout(
    session: Session,
) -> Result<StatusCode, ApiError> {
    info!("User logout request");
    
    // Remove user from session (like req.logout() in Node.js)
    SessionManager::logout(&session).await
        .map_err(|e| ApiError::InternalServerError)?;
    
    info!("User logged out successfully");
    Ok(StatusCode::OK)
}
