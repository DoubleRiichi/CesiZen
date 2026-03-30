use std::result;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::{debug_handler, Json};
use sqlx::PgPool;
use crate::AppState;
use crate::errors::app::AppError;
use crate::modules::feeling::service::FeelingService;
use crate::modules::feeling_tracker::repository::FeelingTrackerRepository;
use crate::modules::feeling_tracker::dto::{FeelingTrackerCreate, FeelingTrackerGet, FeelingTrackerSearchParams};
use crate::modules::feeling_tracker::service::FeelingTrackerService;

#[utoipa::path(
    get,
    path = "/feeling_tracker/{id}",
    tag = "feeling_tracker",
    params(
        ("id" = i32, Path, description = "feeling_tracker ID")
    ))
]
pub async fn get_feeling_tracker_by_id(
    State(pool): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<FeelingTrackerGet>, AppError> {
    FeelingTrackerService::by_id(&pool.db, id).await.map(Json)
}


#[utoipa::path(
    post,
    path = "/feeling_tracker",
    tag = "feeling_tracker",
    request_body = FeelingTrackerCreate,
)]
#[debug_handler]
pub async fn create_feeling_tracker(
    State(pool): State<AppState>,
    Json(body): Json<FeelingTrackerCreate>
) -> Result<Json<i32>, AppError> {

    FeelingTrackerService::create(&pool.db, body)
        .await.map(Json)
}


#[utoipa::path(
    post,
    path = "/feeling_tracker/search",
    tag = "feeling_tracker",
    request_body = FeelingTrackerSearchParams,
)]
#[debug_handler]
pub async fn search_feeling_tracker(
    State(pool): State<AppState>,
    Json(body): Json<FeelingTrackerSearchParams>
) -> Result<Json<Vec<FeelingTrackerGet>>, AppError> {

    FeelingTrackerService::search(&pool.db, body)
        .await.map(Json)
}

#[utoipa::path(
    delete,
    path = "/feeling_tracker/{id}",
    tag = "feeling_tracker",
)]
#[debug_handler]
pub async fn delete_feeling_tracker(
    State(pool): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<()>, AppError> {

    FeelingTrackerService::delete(&pool.db, id)
        .await.map(Json)
}