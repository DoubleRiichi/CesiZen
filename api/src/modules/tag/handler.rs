use crate::errors::app::AppError;
use crate::modules::tag::dto::{TagCreate, TagGet};
use crate::modules::tag::service::TagService;
use crate::AppState;
use axum::extract::{Path, State};
use axum::{debug_handler, Json};

#[utoipa::path(
    get,
    path = "/tag/{id}",
    tag = "tag",
)]
pub async fn get_tag_by_id(
    State(pool): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<TagGet>, AppError> {
    TagService::by_id(&pool.db, id).await.map(Json)
}


#[utoipa::path(
    post,
    path = "/tag",
    tag = "tag",
    request_body = TagCreate,
)]
#[debug_handler]
pub async fn create_tag(
    State(pool): State<AppState>,
    Json(body): Json<TagCreate>
) -> Result<Json<TagGet>, AppError> {

    TagService::create(&pool.db, body)
        .await.map(Json)
}



#[utoipa::path(
    delete,
    path = "/tag/{id}",
    tag = "tag",
)]
#[debug_handler]
pub async fn delete_tag(
    State(pool): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<()>, AppError> {

    TagService::delete(&pool.db, id)
        .await.map(Json)
}

#[utoipa::path(
    get,
    path = "/tag/all",
    tag = "tag",
)]
#[debug_handler]
pub async fn all_tag(
    State(pool): State<AppState>,
) -> Result<Json<Vec<TagGet>>, AppError> {

    TagService::find_all(&pool.db)
        .await.map(Json)
}