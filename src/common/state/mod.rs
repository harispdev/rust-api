use crate::common::database::Database;
use crate::modules::user::repository::UserRepository;
use crate::modules::user::service::UserService;
use crate::modules::auth::repository::AuthRepository;
use crate::modules::auth::service::AuthService;

/// Application state containing shared data
#[derive(Debug, Clone)]
pub struct AppState {
    pub user_service: UserService,
    pub auth_service: AuthService,
}

impl AppState {
    /// Create a new application state
    pub fn new(database: Database) -> Self {
        let user_repository = UserRepository::new(database.connection().clone());
        let user_service = UserService::new(user_repository);
        
        let auth_repository = AuthRepository::new(database.connection().clone());
        let auth_service = AuthService::new(auth_repository);

        Self {
            user_service,
            auth_service,
        }
    }
}