use axum::{
    extract::{Path, State},
    Json,
};
use tracing::info;
use uuid::Uuid;
use validator::Validate;

use crate::{
    common::ApiError,
    modules::user::entity::{CreateUserRequest, UpdateUserRequest, Model as User},
    common::AppState,
};

/// Get all users
pub async fn get_all(State(state): State<AppState>) -> Result<Json<Vec<User>>, ApiError> {
    info!("Fetching all users");
    let result = state.user_service.get_all().await?;
    Ok(Json(result))
}

/// Get a specific user by ID
pub async fn get_by_id(
    Path(id): Path<Uuid>,
    State(state): State<AppState>,
) -> Result<Json<User>, ApiError> {
    info!("Fetching user with ID: {}", id);
    let result = state.user_service.get_by_id(id).await?;
    Ok(Json(result))
}

/// Create a new user
pub async fn create(
    State(state): State<AppState>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<Json<User>, ApiError> {
    info!("Creating new user: {}", payload.email);
    
    // Validate the request
    payload.validate()
        .map_err(|e| ApiError::InvalidInput(format!("Validation error: {}", e)))?;
    
    let result = state.user_service.create(payload).await?;
    Ok(Json(result))
}

/// Update an existing user
pub async fn update(
    Path(id): Path<Uuid>,
    State(state): State<AppState>,
    Json(payload): Json<UpdateUserRequest>,
) -> Result<Json<User>, ApiError> {
    info!("Updating user with ID: {}", id);
    
    // Validate the request
    payload.validate()
        .map_err(|e| ApiError::InvalidInput(format!("Validation error: {}", e)))?;
    
    let result = state.user_service.update(id, payload).await?;
    Ok(Json(result))
}

/// Delete a user
pub async fn delete_user(
    Path(id): Path<Uuid>,
    State(state): State<AppState>,
) -> Result<(), ApiError> {
    info!("Deleting user with ID: {}", id);
    state.user_service.delete(id).await
}

/// Deactivate a user (soft delete)
pub async fn deactivate_user(
    Path(id): Path<Uuid>,
    State(state): State<AppState>,
) -> Result<(), ApiError> {
    info!("Deactivating user with ID: {}", id);
    state.user_service.deactivate(id).await
}

/// Activate a user (restore from soft delete)
pub async fn activate_user(
    Path(id): Path<Uuid>,
    State(state): State<AppState>,
) -> Result<(), ApiError> {
    info!("Activating user with ID: {}", id);
    state.user_service.activate(id).await
}

/// Get users by account ID
pub async fn get_by_account_id(
    Path(account_id): Path<Uuid>,
    State(state): State<AppState>,
) -> Result<Json<Vec<User>>, ApiError> {
    info!("Fetching users by account ID: {}", account_id);
    let result = state.user_service.get_by_account_id(account_id).await?;
    Ok(Json(result))
}

/// Get users by branch ID
pub async fn get_by_branch_id(
    Path(branch_id): Path<Uuid>,
    State(state): State<AppState>,
) -> Result<Json<Vec<User>>, ApiError> {
    info!("Fetching users by branch ID: {}", branch_id);
    let result = state.user_service.get_by_branch_id(branch_id).await?;
    Ok(Json(result))
}

/// Get users by role
pub async fn get_by_role(
    Path(role): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<Vec<User>>, ApiError> {
    info!("Fetching users by role: {}", role);
    let result = state.user_service.get_by_role(&role).await?;
    Ok(Json(result))
}