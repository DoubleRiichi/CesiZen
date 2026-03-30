use std::result;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::{debug_handler, Json};
use sqlx::PgPool;
use crate::AppState;
use crate::errors::app::AppError;
use crate::modules::feeling::dto::{FeelingCreate, FeelingGet, FeelingSearchParams, FeelingUpdate};
use crate::modules::feeling::service::FeelingService;
use crate::modules::feeling_category::dto::{FeelingCategoryCreate, FeelingCategoryGet, FeelingCategorySearchParams};
use crate::modules::feeling_category::repository::FeelingCategoryRepository;
use crate::modules::feeling_category::service::FeelingCategoryService;

#[utoipa::path(
    get,
    path = "/feeling_category/{id}",
    tag = "feeling_category",
    params(
        ("id" = i32, Path, description = "Feeling_Category ID")
    ))
]
pub async fn get_feeling_category_by_id(
    State(pool): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<FeelingCategoryGet>, AppError> {
    FeelingCategoryService::by_id(&pool.db, id).await.map(Json)
}


#[utoipa::path(
    post,
    path = "/feeling_category",
    tag = "feeling_category",
    request_body = FeelingCategoryCreate,
)]
#[debug_handler]
pub async fn create_feeling_category(
    State(pool): State<AppState>,
    Json(body): Json<FeelingCategoryCreate>
) -> Result<Json<i32>, AppError> {

    FeelingCategoryRepository::create(&pool.db, body)
        .await.map(Json)
}


#[utoipa::path(
    post,
    path = "/feeling_category/search",
    tag = "feeling_category",
    request_body = FeelingCategorySearchParams,
)]
#[debug_handler]
pub async fn search_feeling_category(
    State(pool): State<AppState>,
    Json(body): Json<FeelingCategorySearchParams>
) -> Result<Json<Vec<FeelingCategoryGet>>, AppError> {

    FeelingCategoryService::search(&pool.db, body)
        .await.map(Json)
}

#[utoipa::path(
    delete,
    path = "/feeling_category/{id}",
    tag = "feeling_category",
)]
#[debug_handler]
pub async fn delete_feeling_category(
    State(pool): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<()>, AppError> {

    FeelingService::delete(&pool.db, id)
        .await.map(Json)
}