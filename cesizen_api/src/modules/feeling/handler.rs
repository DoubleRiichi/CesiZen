use crate::errors::app::AppError;
use crate::modules::feeling::dto::{FeelingCreate, FeelingGet, FeelingSearchParams, FeelingUpdate};
use crate::modules::feeling::service::FeelingService;
use crate::AppState;
use axum::extract::{Path, State};
use axum::{debug_handler, Json};
use crate::auth::guards::RequireAdmin;

#[utoipa::path(
    get,
    path = "/feeling/{id}",
    tag = "feeling",
    params(
        ("id" = i32, Path, description = "Feeling ID")
    ))
   ]
pub async fn get_feeling_by_id(
    State(pool): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<FeelingGet>, AppError> {
    FeelingService::by_id(&pool.db, id).await.map(Json)
}


#[utoipa::path(
    post,
    path = "/feeling",
    tag = "feeling",
    request_body = FeelingCreate,
)]
#[debug_handler]
pub async fn create_feeling(
    RequireAdmin(_claims): RequireAdmin,
    State(pool): State<AppState>,
    Json(body): Json<FeelingCreate>
) -> Result<Json<i32>, AppError> {

    FeelingService::create(&pool.db, body)
        .await.map(Json)
}

#[utoipa::path(
    put,
    path = "/feeling",
    tag = "feeling",
    request_body = FeelingCreate,
)]
#[debug_handler]
pub async fn update_feeling(
    RequireAdmin(_claims): RequireAdmin,
    State(pool): State<AppState>,
    Path(id): Path<i32>,
    Json(body): Json<FeelingUpdate>
) -> Result<Json<()>, AppError> {

    FeelingService::update(&pool.db, id, body)
        .await.map(Json)
}


#[utoipa::path(
    post,
    path = "/feeling/search",
    tag = "feeling",
    request_body = FeelingSearchParams,
)]
#[debug_handler]
pub async fn search_feeling(
    State(pool): State<AppState>,
    Json(body): Json<FeelingSearchParams>
) -> Result<Json<Vec<FeelingGet>>, AppError> {

    FeelingService::search(&pool.db, body)
        .await.map(Json)
}



#[utoipa::path(
    delete,
    path = "/feeling/{id}",
    tag = "feeling",
)]
#[debug_handler]
pub async fn delete_feeling(
    RequireAdmin(_claims): RequireAdmin,
    State(pool): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<()>, AppError> {

    FeelingService::delete(&pool.db, id)
        .await.map(Json)
}