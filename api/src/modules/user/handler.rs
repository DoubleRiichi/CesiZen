use std::result;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::{debug_handler, Json};
use sqlx::PgPool;
use crate::AppState;
use crate::errors::app::AppError;
use crate::modules::article::dto::ArticleUpdate;
use crate::modules::user::dto::{UserGet, UserCreate, UserUpdate, UserSearchParams};
use crate::modules::user::model::UserRole;
use crate::modules::user::repository::UserRepository;
use crate::modules::user::service::UserService;

#[utoipa::path(
    get,
    path = "/user/:id",
    tag = "user",
)]
pub async fn get_user_by_id(
    State(pool): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<UserGet>, AppError> {
    UserService::by_id(&pool.db, id).await.map(Json)
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
    State(pool): State<AppState>,
    Json(body): Json<UserSearchParams>
) -> Result<Json<Vec<UserGet>>, AppError> {

    UserService::search(&pool.db, body)
        .await.map(Json)
}

#[utoipa::path(
    put,
    path = "/user/:id",
    tag = "user",
    request_body = ArticleUpdate,
)]
#[debug_handler]
pub async fn update_user(
    State(pool): State<AppState>,
    Path(id): Path<i32>,
    Json(body): Json<UserUpdate>
) -> Result<Json<UserGet>, AppError> {

    UserService::update(&pool.db, id, body)
        .await.map(Json)
}


#[utoipa::path(
    delete,
    path = "/user/:id",
    tag = "user",
)]
#[debug_handler]
pub async fn delete_user(
    State(pool): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<()>, AppError> {

    UserService::delete(&pool.db, id)
        .await.map(Json)
}