use crate::errors::app::AppError;
use crate::modules::article::dto::{ArticleCreate, ArticleGet, ArticleSearchParams, ArticleUpdate};
use crate::modules::article::service::ArticleService;
use crate::AppState;
use axum::extract::{Path, State};
use axum::{debug_handler, Json};

#[utoipa::path(
    get,
    path = "/article/{id}",
    tag = "article",
    params(
        ("id" = i32, Path, description = "Article ID")
    ),
    responses(
        (status = 200, description = "Article created", body = ArticleGet),
        (status = 404, description = "Article not found", body = String)
    )
)]
pub async fn get_article_by_id(
    State(pool): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<ArticleGet>, AppError> {
    ArticleService::by_id(&pool.db, id).await.map(Json)
}


#[utoipa::path(
    post,
    path = "/article",
    tag = "article",
    request_body = ArticleCreate,
)]
#[debug_handler]
pub async fn create_article(
    State(pool): State<AppState>,
    Json(body): Json<ArticleCreate>
) -> Result<Json<i32>, AppError> {

    ArticleService::create(&pool.db, body)
        .await.map(Json)
}


#[utoipa::path(
    post,
    path = "/article/search",
    tag = "article",
    request_body = ArticleSearchParams,
)]
#[debug_handler]
pub async fn search_article(
    State(pool): State<AppState>,
    Json(body): Json<ArticleSearchParams>
) -> Result<Json<Vec<ArticleGet>>, AppError> {

    ArticleService::search(&pool.db, body)
        .await.map(Json)
}

#[utoipa::path(
    put,
    path = "/article/{id}",
    tag = "article",
    request_body = ArticleUpdate,
)]
#[debug_handler]
pub async fn update_article(
    State(pool): State<AppState>,
    Path(id): Path<i32>,
    Json(body): Json<ArticleUpdate>
) -> Result<Json<()>, AppError> {

    ArticleService::update(&pool.db, id, body)
        .await.map(Json)
}


#[utoipa::path(
    delete,
    path = "/article/{id}",
    tag = "article",
)]
#[debug_handler]
pub async fn delete_article(
    State(pool): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<()>, AppError> {

    ArticleService::delete(&pool.db, id)
        .await.map(Json)
}