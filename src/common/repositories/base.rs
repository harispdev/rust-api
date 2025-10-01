use anyhow::Result;
use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter, ColumnTrait, Set, ActiveModelTrait, QueryOrder, PaginatorTrait};
use uuid::Uuid;
use tracing::{info, error};

use crate::common::ApiError;

/// Base repository trait with common CRUD operations
#[async_trait::async_trait]
pub trait BaseRepository<T, A, C>
where
    T: EntityTrait + Send + Sync,
    A: ActiveModelTrait + Send + Sync,
    C: ColumnTrait + Send + Sync,
{
    fn db(&self) -> &DatabaseConnection;
    fn entity_name(&self) -> &'static str;

    /// Get all entities
    async fn find_all(&self) -> Result<Vec<T::Model>, ApiError> {
        info!("Fetching all {}s", self.entity_name());
        
        let entities = T::find()
            .all(self.db())
            .await
            .map_err(|e| {
                error!("Failed to fetch {}s: {}", self.entity_name(), e);
                ApiError::DatabaseError(e.to_string())
            })?;

        Ok(entities)
    }

    /// Get entity by ID
    async fn find_by_id(&self, id: Uuid) -> Result<T::Model, ApiError> {
        info!("Fetching {} with ID: {}", self.entity_name(), id);
        
        let entity = T::find_by_id(id)
            .one(self.db())
            .await
            .map_err(|e| {
                error!("Failed to fetch {} with ID {}: {}", self.entity_name(), id, e);
                ApiError::DatabaseError(e.to_string())
            })?;

        match entity {
            Some(entity) => Ok(entity),
            None => Err(ApiError::NotFound(format!("{} not found", self.entity_name()))),
        }
    }

    /// Save entity (create or update)
    async fn save(&self, active_model: A) -> Result<T::Model, ApiError> {
        info!("Saving {}", self.entity_name());
        
        let entity = active_model
            .insert(self.db())
            .await
            .map_err(|e| {
                error!("Failed to save {}: {}", self.entity_name(), e);
                ApiError::DatabaseError(e.to_string())
            })?;

        Ok(entity)
    }

    /// Update entity by ID
    async fn update(&self, id: Uuid, active_model: A) -> Result<T::Model, ApiError> {
        info!("Updating {} with ID: {}", self.entity_name(), id);
        
        // First, get the existing entity
        let _existing = self.find_by_id(id).await?;
        
        let entity = active_model
            .update(self.db())
            .await
            .map_err(|e| {
                error!("Failed to update {} with ID {}: {}", self.entity_name(), id, e);
                ApiError::DatabaseError(e.to_string())
            })?;

        Ok(entity)
    }

    /// Delete entity by ID
    async fn delete(&self, id: Uuid) -> Result<(), ApiError> {
        info!("Deleting {} with ID: {}", self.entity_name(), id);
        
        // First, check if entity exists
        let _existing = self.find_by_id(id).await?;
        
        let result = T::delete_by_id(id)
            .exec(self.db())
            .await
            .map_err(|e| {
                error!("Failed to delete {} with ID {}: {}", self.entity_name(), id, e);
                ApiError::DatabaseError(e.to_string())
            })?;

        if result.rows_affected == 0 {
            return Err(ApiError::NotFound(format!("{} not found", self.entity_name())));
        }

        Ok(())
    }

    /// Check if entity exists by ID
    async fn exists(&self, id: Uuid) -> Result<bool, ApiError> {
        let count = T::find_by_id(id)
            .count(self.db())
            .await
            .map_err(|e| {
                error!("Failed to check if {} exists with ID {}: {}", self.entity_name(), id, e);
                ApiError::DatabaseError(e.to_string())
            })?;

        Ok(count > 0)
    }
}
