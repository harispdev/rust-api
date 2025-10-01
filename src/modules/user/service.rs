use anyhow::Result;
use uuid::Uuid;
use argon2::{Argon2, PasswordHasher};
use argon2::password_hash::{SaltString, rand_core::OsRng};
use tracing::info;

use crate::{
    common::ApiError,
    modules::user::{
        entity::{CreateUserRequest, UpdateUserRequest, Model as User, UserRole},
        repository::UserRepository,
    },
};

/// User service layer for business logic
#[derive(Debug, Clone)]
pub struct UserService {
    repository: UserRepository,
}

impl UserService {
    /// Create a new user service
    pub fn new(repository: UserRepository) -> Self {
        Self { repository }
    }

    /// Get all users
    pub async fn get_all(&self) -> Result<Vec<User>, ApiError> {
        self.repository.get_all().await
    }

    /// Get a user by ID
    pub async fn get_by_id(&self, id: Uuid) -> Result<User, ApiError> {
        self.repository.get_by_id(id).await
    }

    /// Create a new user
    pub async fn create(&self, data: CreateUserRequest) -> Result<User, ApiError> {
        info!("Creating new user: {}", data.email);
        
        // Validate role
        if !self.is_valid_role(&data.role) {
            return Err(ApiError::InvalidInput(format!("Role {} is not valid", data.role)));
        }
        
        // Check if user already exists
        if self.repository.exists_by_email(&data.email).await? {
            return Err(ApiError::UserAlreadyExists);
        }
        
        // Hash the password
        let password_hash = self.hash_password(&data.password)?;
        
        // Create the user
        self.repository.create(data, password_hash).await
    }

    /// Update an existing user
    pub async fn update(&self, id: Uuid, data: UpdateUserRequest) -> Result<User, ApiError> {
        info!("Updating user with ID: {}", id);
        
        // Check if email is being updated and if it already exists
        if let Some(ref email) = data.email {
            if self.repository.exists_by_email(email).await? {
                return Err(ApiError::UserAlreadyExists);
            }
        }
        
        // Hash password if provided
        let password_hash = if let Some(ref password) = data.password {
            Some(self.hash_password(password)?)
        } else {
            None
        };
        
        // Update the user
        self.repository.update(id, data, password_hash).await
    }

    /// Delete a user
    pub async fn delete(&self, id: Uuid) -> Result<(), ApiError> {
        info!("Deleting user with ID: {}", id);
        self.repository.delete(id).await
    }

    /// Deactivate a user (soft delete)
    pub async fn deactivate(&self, id: Uuid) -> Result<(), ApiError> {
        info!("Deactivating user with ID: {}", id);
        
        // Check if user exists
        let user = self.repository.get_by_id(id).await?;
        
        // Don't allow deactivating root users
        if user.role == UserRole::Root.to_string() {
            return Err(ApiError::Unauthorized("Cannot deactivate root users".to_string()));
        }
        
        self.repository.soft_delete(id).await
    }

    /// Activate a user (restore from soft delete)
    pub async fn activate(&self, id: Uuid) -> Result<(), ApiError> {
        info!("Activating user with ID: {}", id);
        self.repository.restore(id).await
    }

    /// Get users by account ID
    pub async fn get_by_account_id(&self, account_id: Uuid) -> Result<Vec<User>, ApiError> {
        self.repository.get_by_account_id(account_id).await
    }

    /// Get users by branch ID
    pub async fn get_by_branch_id(&self, branch_id: Uuid) -> Result<Vec<User>, ApiError> {
        self.repository.get_by_branch_id(branch_id).await
    }

    /// Get users by role
    pub async fn get_by_role(&self, role: &str) -> Result<Vec<User>, ApiError> {
        self.repository.get_by_role(role).await
    }

    /// Hash a password using Argon2
    fn hash_password(&self, password: &str) -> Result<String, ApiError> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        
        let password_hash = argon2
            .hash_password(password.as_bytes(), &salt)
            .map_err(|_e| ApiError::InternalServerError)?;
        
        Ok(password_hash.to_string())
    }

    /// Check if a role is valid
    fn is_valid_role(&self, role: &str) -> bool {
        matches!(role, 
            "ROOT" | "GENERAL_MANAGER" | "MANAGER" | "CUSTOMER" | 
            "WAITER" | "COOK" | "BARMAN" | "CASH_REGISTER"
        )
    }
}