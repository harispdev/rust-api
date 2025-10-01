use axum::{
    extract::{Path, State},
    Json,
};
use tracing::info;

use crate::{
    errors::ApiError,
    models::{CreateUserRequest, UpdateUserRequest, User},
    state::AppState,
};

/// Get all users
pub async fn get_users(State(state): State<AppState>) -> Json<Vec<User>> {
    info!("Fetching all users");
    
    let users: Vec<User> = state.users.read().await.values().cloned().collect();
    
    Json(users)
}

/// Get a specific user by ID
pub async fn get_user(
    Path(id): Path<u32>,
    State(state): State<AppState>,
) -> Result<Json<User>, ApiError> {
    info!("Fetching user with ID: {}", id);
    
    let users = state.users.read().await;
    let user = users.get(&id)
        .ok_or(ApiError::UserNotFound)?;
    
    Ok(Json(user.clone()))
}

/// Create a new user
pub async fn create_user(
    State(state): State<AppState>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<Json<User>, ApiError> {
    info!("Creating new user: {}", payload.email);
    
    // Validate the request
    payload.validate()
        .map_err(ApiError::InvalidInput)?;
    
    // Generate new ID
    let id = {
        let users = state.users.read().await;
        users.len() as u32 + 1
    };
    
    // Create the user
    let user = User::new(id, payload.name, payload.email);
    
    // Store the user
    {
        let mut users = state.users.write().await;
        users.insert(id, user.clone());
    }
    
    info!("Created user with ID: {}", id);
    Ok(Json(user))
}

/// Update an existing user
pub async fn update_user(
    Path(id): Path<u32>,
    State(state): State<AppState>,
    Json(payload): Json<UpdateUserRequest>,
) -> Result<Json<User>, ApiError> {
    info!("Updating user with ID: {}", id);
    
    // Validate the request
    payload.validate()
        .map_err(ApiError::InvalidInput)?;
    
    let mut users = state.users.write().await;
    let user = users.get_mut(&id)
        .ok_or(ApiError::UserNotFound)?;
    
    // Update fields if provided
    if let Some(name) = payload.name {
        user.name = name;
    }
    
    if let Some(email) = payload.email {
        user.email = email;
    }
    
    let updated_user = user.clone();
    drop(users); // Release the lock
    
    info!("Updated user with ID: {}", id);
    Ok(Json(updated_user))
}

/// Delete a user
pub async fn delete_user(
    Path(id): Path<u32>,
    State(state): State<AppState>,
) -> Result<(), ApiError> {
    info!("Deleting user with ID: {}", id);
    
    let mut users = state.users.write().await;
    users.remove(&id)
        .ok_or(ApiError::UserNotFound)?;
    
    info!("Deleted user with ID: {}", id);
    Ok(())
}