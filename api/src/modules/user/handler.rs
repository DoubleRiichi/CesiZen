use crate::errors::app::AppError;
use crate::modules::user::dto::{UserCreate, UserGet, UserSearchParams, UserUpdate};
use crate::modules::user::repository::UserRepository;
use crate::modules::user::service::UserService;
use crate::AppState;
use axum::extract::{Path, State};
use axum::{debug_handler, Json};

#[utoipa::path(
    get,
    path = "/user/{id}",
    tag = "user",
)]
pub async fn get_user_by_id(
    RequireAuth(claims): RequireAuth,
    State(pool): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<UserGet>, AppError> {
    let result = UserService::by_id(&pool.db, id).await?;

    assert_owns_resource(&claims, result.id)?;

    Ok(Json(result))
}


#[utoipa::path(
    post,
    path = "/user",
    tag = "user",
    request_body = UserCreate,
)]
#[debug_handler]
pub async fn create_user(
    State(pool): State<AppState>,
    Json(body): Json<UserCreate>
) -> Result<Json<UserGet>, AppError> {

    let result = UserService::create(&pool.db, body)
        .await?;

    Ok(result.into())
}




#[utoipa::path(
    post,
    path = "/user/search",
    tag = "user",
    request_body = UserSearchParams,
)]
#[debug_handler]
pub async fn search_user(
    RequireAdmin(_claims): RequireAdmin,
    State(pool): State<AppState>,
    Json(body): Json<UserSearchParams>
) -> Result<Json<Vec<UserGet>>, AppError> {

    UserService::search(&pool.db, body)
        .await.map(Json)
}

#[utoipa::path(
    put,
    path = "/user/{id}",
    tag = "user",
    request_body = UserUpdate,
)]
#[debug_handler]
pub async fn update_user(
    RequireAuth(claims): RequireAuth,
    State(pool): State<AppState>,
    Path(id): Path<i32>,
    Json(body): Json<UserUpdate>
) -> Result<Json<UserGet>, AppError> {

    assert_owns_resource(&claims, id)?;

    UserService::update(&pool.db, id, body)
        .await.map(Json)
}


#[utoipa::path(
    delete,
    path = "/user/{id}",
    tag = "user",
)]
#[debug_handler]
pub async fn delete_user(
    RequireAdmin(_claims): RequireAdmin,
    State(pool): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<()>, AppError> {

    UserService::delete(&pool.db, id)
        .await.map(Json)
}


use crate::auth::guards::{assert_owns_resource, RequireAdmin, RequireAuth};
use crate::auth::{claims::Claims, encode_jwt};
use crate::modules::user::dto::{LoginRequest, LoginResponse};
#[utoipa::path(
    post,
    path = "/user/login",
    tag = "user",
)]
#[debug_handler]
pub async fn login(
    State(pool): State<AppState>,
    Json(body): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, AppError> {

    let user_row = UserRepository::by_mail(&pool.db, &body.email)
        .await
        .map_err(|_| AppError::Validation("Invalid email or password".to_string()))?;

    let valid = bcrypt::verify(&body.password, &user_row.password)
        .map_err(|_| AppError::Internal("Password verification failed".to_string()))?;

    if !valid {
        return Err(AppError::Validation("Invalid email or password".to_string()));
    }

    let claims = Claims::new(user_row.id, user_row.email.clone(), user_row.role.clone());
    let token = encode_jwt(&claims)?;

    Ok(Json(LoginResponse {
        token,
        user: user_row.into(),
    }))
}