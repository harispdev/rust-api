use anyhow::Result;
use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter, ColumnTrait, PaginatorTrait};
use tracing::{info, error};

use crate::{
    modules::user::entity::{Entity as UserEntity, Column},
    common::ApiError,
};

/// Auth repository for authentication-related database operations
#[derive(Debug, Clone)]
pub struct AuthRepository {
    db: DatabaseConnection,
}

impl AuthRepository {
    /// Create a new auth repository
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    /// Find user by email for authentication
    pub async fn find_user_by_email(&self, email: &str) -> Result<Option<crate::modules::user::entity::Model>, ApiError> {
        info!("Finding user by email: {}", email);
        
        let user = UserEntity::find()
            .filter(Column::Email.eq(email))
            .one(&self.db)
            .await
            .map_err(|e| {
                error!("Failed to find user by email {}: {}", email, e);
                ApiError::DatabaseError(e.to_string())
            })?;

        Ok(user)
    }

    /// Check if user exists by email
    pub async fn user_exists_by_email(&self, email: &str) -> Result<bool, ApiError> {
        info!("Checking if user exists by email: {}", email);
        
        let count = UserEntity::find()
            .filter(Column::Email.eq(email))
            .count(&self.db)
            .await
            .map_err(|e| {
                error!("Failed to check if user exists with email {}: {}", email, e);
                ApiError::DatabaseError(e.to_string())
            })?;

        Ok(count > 0)
    }
}
