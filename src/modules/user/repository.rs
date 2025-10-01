use anyhow::Result;
use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter, ColumnTrait, Set, ActiveModelTrait, QueryOrder, PaginatorTrait};
use uuid::Uuid;
use tracing::{info, error};

use crate::{
    modules::user::entity::{Entity as UserEntity, Model as User, CreateUserRequest, UpdateUserRequest, Column, ActiveModel},
    common::ApiError,
};

/// User repository for database operations
#[derive(Debug, Clone)]
pub struct UserRepository {
    db: DatabaseConnection,
}

impl UserRepository {
    /// Create a new user repository
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    /// Get all users with ordering
    pub async fn get_all(&self) -> Result<Vec<User>, ApiError> {
        info!("Fetching all users from database");
        
        let users = UserEntity::find()
            .order_by_desc(Column::CreatedAt)
            .all(&self.db)
            .await
            .map_err(|e| {
                error!("Failed to fetch users: {}", e);
                ApiError::DatabaseError(e.to_string())
            })?;

        Ok(users)
    }

    /// Get a user by ID
    pub async fn get_by_id(&self, id: Uuid) -> Result<User, ApiError> {
        info!("Fetching user with ID: {}", id);

        let user = UserEntity::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(|e| {
                error!("Failed to fetch user with ID {}: {}", id, e);
                ApiError::DatabaseError(e.to_string())
            })?;

        match user {
            Some(user) => Ok(user),
            None => Err(ApiError::UserNotFound),
        }
    }

    /// Create a new user
    pub async fn create(&self, request: CreateUserRequest, password_hash: String) -> Result<User, ApiError> {
        info!("Creating new user: {}", request.email);

        let now = chrono::Utc::now().fixed_offset();
        let user = ActiveModel {
            id: Set(uuid::Uuid::new_v4()),
            account_id: Set(request.account_id),
            branch_id: Set(request.branch_id),
            name: Set(request.name),
            email: Set(request.email),
            password_hash: Set(Some(password_hash)),
            role: Set(request.role),
            status: Set("ACTIVE".to_string()),
            created_at: Set(now),
            updated_at: Set(now),
            deleted_at: Set(None),
        };

        let user = user.insert(&self.db)
            .await
            .map_err(|e| {
                error!("Failed to create user: {}", e);
                match e {
                    sea_orm::error::DbErr::RecordNotInserted => ApiError::UserAlreadyExists,
                    _ => ApiError::DatabaseError(e.to_string()),
                }
            })?;

        info!("Created user with ID: {}", user.id);
        Ok(user)
    }

    /// Update an existing user
    pub async fn update(&self, id: Uuid, request: UpdateUserRequest, password_hash: Option<String>) -> Result<User, ApiError> {
        info!("Updating user with ID: {}", id);

        // First, get the existing user
        let user = self.get_by_id(id).await?;

        // Create active model for update
        let mut user: ActiveModel = user.into();

        // Update fields if provided
        if let Some(name) = request.name {
            user.name = Set(Some(name));
        }

        if let Some(email) = request.email {
            user.email = Set(email);
        }

        if let Some(hash) = password_hash {
            user.password_hash = Set(Some(hash));
        }

        // Update timestamp
        user.updated_at = Set(chrono::Utc::now().fixed_offset());

        let user = user.update(&self.db)
            .await
            .map_err(|e| {
                error!("Failed to update user {}: {}", id, e);
                match e {
                    sea_orm::error::DbErr::RecordNotUpdated => ApiError::UserAlreadyExists,
                    _ => ApiError::DatabaseError(e.to_string()),
                }
            })?;

        info!("Updated user with ID: {}", id);
        Ok(user)
    }

    /// Delete a user by ID
    pub async fn delete(&self, id: Uuid) -> Result<(), ApiError> {
        info!("Deleting user with ID: {}", id);
        
        // First, check if user exists
        let _existing = self.get_by_id(id).await?;
        
        let result = UserEntity::delete_by_id(id)
            .exec(&self.db)
            .await
            .map_err(|e| {
                error!("Failed to delete user with ID {}: {}", id, e);
                ApiError::DatabaseError(e.to_string())
            })?;

        if result.rows_affected == 0 {
            return Err(ApiError::UserNotFound);
        }

        info!("Deleted user with ID: {}", id);
        Ok(())
    }

    /// Check if a user exists by email
    pub async fn exists_by_email(&self, email: &str) -> Result<bool, ApiError> {
        let count = UserEntity::find()
            .filter(Column::Email.eq(email))
            .filter(Column::DeletedAt.is_null())
            .count(&self.db)
            .await
            .map_err(|e| {
                error!("Failed to check if user exists with email {}: {}", email, e);
                ApiError::DatabaseError(e.to_string())
            })?;

        Ok(count > 0)
    }

    /// Soft delete a user
    pub async fn soft_delete(&self, id: Uuid) -> Result<(), ApiError> {
        info!("Soft deleting user with ID: {}", id);
        
        let now = chrono::Utc::now().fixed_offset();
        let mut user: ActiveModel = self.get_by_id(id).await?.into();
        user.deleted_at = Set(Some(now));
        user.updated_at = Set(now);
        
        user.update(&self.db)
            .await
            .map_err(|e| {
                error!("Failed to soft delete user with ID {}: {}", id, e);
                ApiError::DatabaseError(e.to_string())
            })?;

        info!("Soft deleted user with ID: {}", id);
        Ok(())
    }

    /// Restore a soft-deleted user
    pub async fn restore(&self, id: Uuid) -> Result<(), ApiError> {
        info!("Restoring user with ID: {}", id);
        
        let now = chrono::Utc::now().fixed_offset();
        let mut user: ActiveModel = self.get_by_id(id).await?.into();
        user.deleted_at = Set(None);
        user.updated_at = Set(now);
        
        user.update(&self.db)
            .await
            .map_err(|e| {
                error!("Failed to restore user with ID {}: {}", id, e);
                ApiError::DatabaseError(e.to_string())
            })?;

        info!("Restored user with ID: {}", id);
        Ok(())
    }

    /// Get users by account ID
    pub async fn get_by_account_id(&self, account_id: Uuid) -> Result<Vec<User>, ApiError> {
        info!("Fetching users by account ID: {}", account_id);
        
        let users = UserEntity::find()
            .filter(Column::AccountId.eq(account_id))
            .filter(Column::DeletedAt.is_null())
            .order_by_desc(Column::CreatedAt)
            .all(&self.db)
            .await
            .map_err(|e| {
                error!("Failed to fetch users by account ID {}: {}", account_id, e);
                ApiError::DatabaseError(e.to_string())
            })?;

        Ok(users)
    }

    /// Get users by branch ID
    pub async fn get_by_branch_id(&self, branch_id: Uuid) -> Result<Vec<User>, ApiError> {
        info!("Fetching users by branch ID: {}", branch_id);
        
        let users = UserEntity::find()
            .filter(Column::BranchId.eq(branch_id))
            .filter(Column::DeletedAt.is_null())
            .order_by_desc(Column::CreatedAt)
            .all(&self.db)
            .await
            .map_err(|e| {
                error!("Failed to fetch users by branch ID {}: {}", branch_id, e);
                ApiError::DatabaseError(e.to_string())
            })?;

        Ok(users)
    }

    /// Get users by role
    pub async fn get_by_role(&self, role: &str) -> Result<Vec<User>, ApiError> {
        info!("Fetching users by role: {}", role);
        
        let users = UserEntity::find()
            .filter(Column::Role.eq(role))
            .filter(Column::DeletedAt.is_null())
            .order_by_desc(Column::CreatedAt)
            .all(&self.db)
            .await
            .map_err(|e| {
                error!("Failed to fetch users by role {}: {}", role, e);
                ApiError::DatabaseError(e.to_string())
            })?;

        Ok(users)
    }

}