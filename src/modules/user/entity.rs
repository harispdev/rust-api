use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize, Serializer};
use uuid::Uuid;
use validator::Validate;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Deserialize)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub account_id: Uuid,
    pub branch_id: Option<Uuid>,
    pub name: Option<String>,
    pub email: String,
    #[serde(skip_serializing)]
    pub password_hash: Option<String>,
    pub role: String,
    pub status: String,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
    pub deleted_at: Option<DateTimeWithTimeZone>,
}

impl Serialize for Model {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("User", 8)?;
        state.serialize_field("id", &self.id)?;
        state.serialize_field("account_id", &self.account_id)?;
        state.serialize_field("branch_id", &self.branch_id)?;
        state.serialize_field("name", &self.name)?;
        state.serialize_field("email", &self.email)?;
        state.serialize_field("role", &self.role)?;
        state.serialize_field("status", &self.status)?;
        state.serialize_field("created_at", &self.created_at)?;
        state.serialize_field("updated_at", &self.updated_at)?;
        state.end()
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

// Enums
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UserStatus {
    Active,
    Inactive,
}

impl std::fmt::Display for UserStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserStatus::Active => write!(f, "ACTIVE"),
            UserStatus::Inactive => write!(f, "INACTIVE"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UserRole {
    Root,
    GeneralManager,
    Manager,
    Customer,
    Waiter,
    Cook,
    Barman,
    CashRegister,
}

impl std::fmt::Display for UserRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserRole::Root => write!(f, "ROOT"),
            UserRole::GeneralManager => write!(f, "GENERAL_MANAGER"),
            UserRole::Manager => write!(f, "MANAGER"),
            UserRole::Customer => write!(f, "CUSTOMER"),
            UserRole::Waiter => write!(f, "WAITER"),
            UserRole::Cook => write!(f, "COOK"),
            UserRole::Barman => write!(f, "BARMAN"),
            UserRole::CashRegister => write!(f, "CASH_REGISTER"),
        }
    }
}

// Request/Response DTOs
#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct CreateUserRequest {
    pub account_id: Uuid,
    pub branch_id: Option<Uuid>,
    #[validate(length(min = 1, max = 100, message = "Name must be between 1 and 100 characters"))]
    pub name: Option<String>,
    
    #[validate(email(message = "Invalid email format"))]
    pub email: String,
    
    #[validate(length(min = 8, max = 100, message = "Password must be between 8 and 100 characters"))]
    pub password: String,
    
    pub role: String,
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct UpdateUserRequest {
    pub branch_id: Option<Uuid>,
    #[validate(length(min = 1, max = 100, message = "Name must be between 1 and 100 characters"))]
    pub name: Option<String>,
    
    #[validate(email(message = "Invalid email format"))]
    pub email: Option<String>,
    
    #[validate(length(min = 8, max = 100, message = "Password must be between 8 and 100 characters"))]
    pub password: Option<String>,
    
    pub role: Option<String>,
    pub status: Option<String>,
}

// UserResponse is now just Model with custom serialization that excludes password_hash
