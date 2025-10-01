use serde::{Deserialize, Serialize};
use validator::Validate;

/// Login request DTO
#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct LoginRequest {
    #[validate(email(message = "Invalid email format"))]
    pub email: String,
    
    #[validate(length(min = 1, message = "Password is required"))]
    pub password: String,
}

/// User information for session context
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserInfo {
    pub id: String,
    pub account_id: String,
    pub branch_id: Option<String>,
    pub name: Option<String>,
    pub email: String,
    pub role: String,
    pub status: String,
}

impl From<crate::modules::user::entity::Model> for UserInfo {
    fn from(user: crate::modules::user::entity::Model) -> Self {
        Self {
            id: user.id.to_string(),
            account_id: user.account_id.to_string(),
            branch_id: user.branch_id.map(|id| id.to_string()),
            name: user.name,
            email: user.email,
            role: user.role,
            status: user.status,
        }
    }
}
