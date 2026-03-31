use crate::modules::user::model::UserRole;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: i32,           // user id
    pub email: String,
    pub role: UserRole,
    pub exp: usize,         // expiration (Unix timestamp)
    pub iat: usize,         // issued at
}

impl Claims {
    pub fn new(user_id: i32, email: String, role: UserRole) -> Self {
        let now = chrono::Utc::now();
        Self {
            sub: user_id,
            email,
            role,
            iat: now.timestamp() as usize,
            exp: (now + chrono::Duration::hours(24)).timestamp() as usize,
        }
    }
}