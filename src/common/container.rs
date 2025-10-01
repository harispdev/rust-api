use std::sync::Arc;
use sea_orm::DatabaseConnection;

use crate::{
    common::repositories::base::BaseRepository,
    modules::user::repository::UserRepository,
    modules::user::service::UserService,
};

/// Dependency injection container
#[derive(Debug, Clone)]
pub struct Container {
    pub user_repository: Arc<UserRepository>,
    pub user_service: Arc<UserService>,
}

impl Container {
    /// Create a new container with all dependencies
    pub fn new(db: DatabaseConnection) -> Self {
        let user_repository = Arc::new(UserRepository::new(db.clone()));
        let user_service = Arc::new(UserService::new(user_repository.clone()));

        Self {
            user_repository,
            user_service,
        }
    }

    /// Get user service
    pub fn get_user_service(&self) -> Arc<UserService> {
        self.user_service.clone()
    }

    /// Get user repository
    pub fn get_user_repository(&self) -> Arc<UserRepository> {
        self.user_repository.clone()
    }
}
