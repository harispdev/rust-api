use serde::{Deserialize, Serialize};
use std::fmt;

/// Represents a user in the system
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct User {
    pub id: u32,
    pub name: String,
    pub email: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl User {
    /// Create a new user with the given details
    pub fn new(id: u32, name: String, email: String) -> Self {
        Self {
            id,
            name,
            email,
            created_at: chrono::Utc::now(),
        }
    }
}

impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "User {{ id: {}, name: {}, email: {} }}", self.id, self.name, self.email)
    }
}

/// Request payload for creating a new user
#[derive(Debug, Deserialize)]
pub struct CreateUserRequest {
    pub name: String,
    pub email: String,
}

impl CreateUserRequest {
    /// Validate the request data
    pub fn validate(&self) -> Result<(), String> {
        if self.name.trim().is_empty() {
            return Err("Name cannot be empty".to_string());
        }
        
        if self.email.trim().is_empty() {
            return Err("Email cannot be empty".to_string());
        }
        
        if !self.email.contains('@') {
            return Err("Invalid email format".to_string());
        }
        
        Ok(())
    }
}

/// Request payload for updating a user
#[derive(Debug, Deserialize)]
pub struct UpdateUserRequest {
    pub name: Option<String>,
    pub email: Option<String>,
}

impl UpdateUserRequest {
    /// Validate the request data
    pub fn validate(&self) -> Result<(), String> {
        if let Some(name) = &self.name {
            if name.trim().is_empty() {
                return Err("Name cannot be empty".to_string());
            }
        }
        
        if let Some(email) = &self.email {
            if email.trim().is_empty() {
                return Err("Email cannot be empty".to_string());
            }
            
            if !email.contains('@') {
                return Err("Invalid email format".to_string());
            }
        }
        
        Ok(())
    }
}