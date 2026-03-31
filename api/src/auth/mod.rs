pub mod claims;
pub mod middleware;
pub mod guards;

use crate::auth::claims::Claims;
use crate::errors::app::AppError;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};

pub fn jwt_secret() -> String {
    std::env::var("JWT_SECRET").expect("JWT_SECRET must be set")
}

pub fn encode_jwt(claims: &Claims) -> Result<String, AppError> {
    encode(
        &Header::default(),
        claims,
        &EncodingKey::from_secret(jwt_secret().as_bytes()),
    )
        .map_err(|e| AppError::Internal(format!("JWT encode error: {}", e)))
}

pub fn decode_jwt(token: &str) -> Result<Claims, AppError> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(jwt_secret().as_bytes()),
        &Validation::default(),
    )
        .map(|data| data.claims)
        .map_err(|e| AppError::Validation(format!("Invalid token: {}", e)))
}