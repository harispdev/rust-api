use axum::{
    extract::Request,
    http::StatusCode,
    middleware::Next,
    response::Response,
};
use tower_sessions::Session;

use crate::{
    common::session::SessionManager,
    modules::auth::entity::UserInfo,
};

/// Authentication middleware that checks if user is authenticated
pub async fn authenticate(
    mut request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    // Extract session from request extensions
    let session = request.extensions().get::<Session>()
        .ok_or(StatusCode::UNAUTHORIZED)?;
    
    // Check if user is logged in via session
    if let Some(user_info) = SessionManager::get_current_user(session).await {
        // Set user context in request extensions (like your Node.js implementation)
        request.extensions_mut().insert(user_info);
        Ok(next.run(request).await)
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}

/// Authorization middleware that checks user roles
pub fn authorize(roles: Vec<&'static str>) -> impl Fn(Request, Next) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<Response, StatusCode>> + Send + 'static>> {
    move |request: Request, next: Next| {
        let roles = roles.clone();
        Box::pin(async move {
            // Get user from request context
            if let Some(user) = request.extensions().get::<UserInfo>() {
                // Check if user role is in allowed roles
                if roles.contains(&"ALL") || roles.contains(&user.role.as_str()) {
                    return Ok(next.run(request).await);
                }
            }
            
            Err(StatusCode::FORBIDDEN)
        })
    }
}

/// Set user request context (similar to your Node.js implementation)
pub async fn set_user_request_context(
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    // This would typically be handled by session middleware
    // For now, we'll just pass through
    Ok(next.run(request).await)
}

/// Extract user info from request extensions
pub fn extract_user_info(request: &Request) -> Option<&UserInfo> {
    request.extensions().get::<UserInfo>()
}
