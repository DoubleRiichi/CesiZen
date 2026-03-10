use std::option::Option;
use serde::{Serialize, Deserialize};
use utoipa::ToSchema;
use validator::Validate;
use crate::modules::user::model::*;

#[derive(Serialize, Debug, ToSchema)]
pub struct UserGet {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub age: i16,
    pub avatar: String,
    pub is_active: bool,
    pub role: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

impl From<(UserRow)> for UserGet {
    fn from(user: UserRow) -> Self {
        Self {
            id: user.id,
            username: user.username,
            email: user.email,
            age: user.age,
            avatar: user.avatar,
            is_active: user.is_active,
            role: user.role.to_string(),
            created_at: user.created_at,
            updated_at: user.updated_at,
        }
    }
}


#[derive(Serialize, Debug, ToSchema)]
pub struct UserGetSimple {
    pub id: i32,
    pub username: String,
    pub avatar: String,
    pub role: String,
}

impl From<UserRow> for UserGetSimple {
    fn from(user: (UserRow)) -> Self {
        Self {
            id: user.id,
            username: user.username,
            avatar: user.avatar,
            role: user.role.to_string(),
        }
    }
}


#[derive(Deserialize, Debug, Validate, ToSchema)]
pub struct UserCreate {
    #[validate(length(min = 4, max = 20))]
    pub username: String,
    #[validate(length(min = 8, max = 32))]
    pub password: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 6, max = 80))]
    pub avatar: String,
    #[validate(range(min = 13, max = 120))]
    pub age: i16,
}

#[derive(Deserialize, Debug, Validate, ToSchema)]
pub struct UserUpdate {
    #[validate(length(min = 8, max = 32))]
    pub password: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 6, max = 80))]
    pub avatar: String,
    pub is_active: bool,
}



#[derive(Debug, Deserialize, ToSchema)]
pub struct UserSearchParams {
    pub username: Option<String>,
    pub email: Option<String>,
    pub age: Option<i16>,
    pub avatar: Option<String>,
    pub is_active: Option<bool>,
    pub role: Option<UserRole>,
    pub start_at: Option<chrono::NaiveDateTime>,
    pub end_at: Option<chrono::NaiveDateTime>,
    pub cursor: Option<chrono::NaiveDateTime>,
    pub page_size: Option<i32>,
}