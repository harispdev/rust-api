use axum::{
    extract::{FromRequestParts, State},
    http::{request::Parts, StatusCode},
    response::Response,
};
use tower_sessions::{
    Session, SessionManagerLayer, SessionStore,
};
use tower_sessions_redis_store::RedisStore;
use fred::clients::RedisClient;
use fred::interfaces::ClientLike;
use serde::{Deserialize, Serialize};

use crate::{
    common::config::SessionConfig,
    modules::auth::entity::UserInfo,
};

/// Session data stored in the session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionData {
    pub user: UserInfo,
}

/// Session store type
pub type SessionStoreType = RedisStore<RedisClient>;

/// Create session layer with Redis store
pub async fn create_session_layer(config: &SessionConfig) -> SessionManagerLayer<SessionStoreType> {
    let redis_client = RedisClient::new(
        fred::types::RedisConfig::from_url(&config.redis_url)
            .expect("Failed to parse Redis URL"),
        None,
        None,
        None,
    );
    
    redis_client.connect();
    redis_client.wait_for_connect().await.expect("Failed to connect to Redis");
    
    let store = RedisStore::new(redis_client);
    
    SessionManagerLayer::new(store)
        .with_name(&config.cookie_name)
        .with_secure(config.cookie_secure)
        .with_same_site(parse_same_site(&config.cookie_same_site))
        .with_http_only(true) // HTTP-only cookies for security
}

/// Parse SameSite cookie attribute
fn parse_same_site(same_site: &str) -> tower_sessions::cookie::SameSite {
    match same_site.to_lowercase().as_str() {
        "strict" => tower_sessions::cookie::SameSite::Strict,
        "lax" => tower_sessions::cookie::SameSite::Lax,
        "none" => tower_sessions::cookie::SameSite::None,
        _ => tower_sessions::cookie::SameSite::Lax,
    }
}

/// Extract user from session
#[derive(Debug, Clone)]
pub struct SessionUser(pub UserInfo);

#[axum::async_trait]
impl<S> FromRequestParts<S> for SessionUser
where
    S: Send + Sync,
{
    type Rejection = StatusCode;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let session = parts
            .extensions
            .get::<Session>()
            .ok_or(StatusCode::UNAUTHORIZED)?;

        let user_data: SessionData = session
            .get("user")
            .await
            .map_err(|_| StatusCode::UNAUTHORIZED)?
            .ok_or(StatusCode::UNAUTHORIZED)?;

        Ok(SessionUser(user_data.user))
    }
}

/// Session management utilities
pub struct SessionManager;

impl SessionManager {
    /// Login user by storing user info in session
    pub async fn login(session: &Session, user: UserInfo) -> Result<(), tower_sessions::session::Error> {
        let session_data = SessionData { user };
        session.insert("user", session_data).await
    }

    /// Logout user by removing user info from session
    pub async fn logout(session: &Session) -> Result<Option<SessionData>, tower_sessions::session::Error> {
        session.remove::<SessionData>("user").await
    }

    /// Check if user is logged in
    pub async fn is_logged_in(session: &Session) -> bool {
        session.get::<SessionData>("user").await.is_ok()
    }

    /// Get current user from session
    pub async fn get_current_user(session: &Session) -> Option<UserInfo> {
        if let Ok(Some(session_data)) = session.get::<SessionData>("user").await {
            Some(session_data.user)
        } else {
            None
        }
    }
}
