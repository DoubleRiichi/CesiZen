use crate::auth::claims::Claims;
use crate::errors::app::AppError;
use crate::modules::user::model::UserRole;
use crate::AppState;
use axum::{
    extract::FromRequestParts,
    http::request::Parts,
};

/// Guard : requiert le rôle Admin
pub struct RequireAdmin(pub Claims);

impl FromRequestParts<AppState> for RequireAdmin {
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &AppState) -> Result<Self, Self::Rejection> {
        let claims = Claims::from_request_parts(parts, state).await?;
        match claims.role {
            UserRole::Admin => Ok(RequireAdmin(claims)),
            _ => Err(AppError::Validation("Admin role required".to_string())),
        }
    }
}

pub struct RequireMod(pub Claims);

impl FromRequestParts<AppState> for RequireMod {
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &AppState) -> Result<Self, Self::Rejection> {
        let claims = Claims::from_request_parts(parts, state).await?;
        match claims.role {
            UserRole::Admin | UserRole::Mod => Ok(RequireMod(claims)),
            _ => Err(AppError::Validation("Moderator role required".to_string())),
        }
    }
}

/// Guard : tout utilisateur authentifié
pub struct RequireAuth(pub Claims);

impl FromRequestParts<AppState> for RequireAuth {
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &AppState) -> Result<Self, Self::Rejection> {
        let claims = Claims::from_request_parts(parts, state).await?;
        Ok(RequireAuth(claims))
    }
}



pub fn assert_owns_resource(claims: &Claims, resource_owner_id: i32) -> Result<(), AppError> {
    match claims.role {
        UserRole::Admin => Ok(()),
        _ if claims.sub == resource_owner_id => Ok(()),
        _ => Err(AppError::Forbidden(
            format!("Resource belongs to user {}, not you", resource_owner_id)
        )),
    }
}