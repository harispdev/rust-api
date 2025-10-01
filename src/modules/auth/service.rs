use anyhow::Result;
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use tracing::info;

use crate::{
    common::ApiError,
    modules::{
        auth::{
            entity::{LoginRequest, UserInfo},
            repository::AuthRepository,
        },
        user::entity::UserStatus,
    },
};

/// Auth service for authentication business logic
#[derive(Debug, Clone)]
pub struct AuthService {
    repository: AuthRepository,
}

impl AuthService {
    /// Create a new auth service
    pub fn new(repository: AuthRepository) -> Self {
        Self { repository }
    }

    /// Login an existing user
    pub async fn login(&self, request: LoginRequest) -> Result<UserInfo, ApiError> {
        info!("Attempting login for user: {}", request.email);
        
        // Find user by email
        let user = self.repository.find_user_by_email(&request.email).await?
            .ok_or(ApiError::InvalidCredentials)?;
        
        // Check if user is active
        if user.status != UserStatus::Active.to_string() {
            return Err(ApiError::Unauthorized("User is not active".to_string()));
        }
        
        // Verify password if it exists (some users might not have passwords)
        if let Some(ref password_hash) = user.password_hash {
            self.verify_password(&request.password, password_hash)?;
        } else {
            return Err(ApiError::InvalidCredentials);
        }
        
        info!("User logged in successfully: {}", user.email);
        
        Ok(UserInfo::from(user))
    }

    /// Verify a password against its hash
    fn verify_password(&self, password: &str, hash: &str) -> Result<(), ApiError> {
        let parsed_hash = PasswordHash::new(hash)
            .map_err(|_| ApiError::InvalidCredentials)?;
        
        Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .map_err(|_| ApiError::InvalidCredentials)?;
        
        Ok(())
    }
}
