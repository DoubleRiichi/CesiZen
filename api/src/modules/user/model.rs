use std::fmt;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(sqlx::FromRow)]
pub struct UserRow {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
    pub role: UserRole,
    pub age: i16,
    pub avatar: String,
    pub is_active: bool,
    pub created_at: chrono::DateTime<Utc>,
    pub updated_at: chrono::DateTime<Utc>,
}

#[derive(sqlx::Type, Serialize, Deserialize, Debug, ToSchema, Clone)]
#[sqlx(type_name = "user_role", rename_all = "PascalCase")]
pub enum UserRole {
    Admin,
    Mod,
    User,
}

impl fmt::Display for UserRole {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UserRole::Admin => write!(f, "Admin"),
            UserRole::Mod => write!(f, "Mod"),
            UserRole::User => write!(f, "User"),
        }
    }
}