use axum::{
    extract::FromRequestParts,
    http::{request::Parts, header},
};
use crate::auth::{claims::Claims, decode_jwt};
use crate::errors::app::AppError;
use crate::AppState;

impl FromRequestParts<AppState> for Claims {
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut Parts,
        _state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        // Lire le header Authorization: Bearer <token>
        let auth_header = parts
            .headers
            .get(header::AUTHORIZATION)
            .and_then(|v| v.to_str().ok())
            .ok_or_else(|| AppError::Validation("Missing Authorization header".to_string()))?;

        let token = auth_header
            .strip_prefix("Bearer ")
            .ok_or_else(|| AppError::Validation("Authorization header must start with 'Bearer '".to_string()))?;

        decode_jwt(token)
    }
}