use crate::errors::app::AppError;
use crate::modules::feeling_tracker::dto::{FeelingTrackerCreate, FeelingTrackerGet, FeelingTrackerSearchParams, FeelingTrackerUpdate};
use crate::modules::feeling_tracker::service::FeelingTrackerService;
use crate::AppState;
use axum::extract::{Path, State};
use axum::{debug_handler, Json};
use crate::auth::guards::{assert_owns_resource, RequireAdmin, RequireAuth};

#[utoipa::path(
    get,
    path = "/feeling_tracker/{id}",
    tag = "feeling_tracker",
    params(
        ("id" = i32, Path, description = "feeling_tracker ID")
    ))
]
pub async fn get_feeling_tracker_by_id(
    RequireAuth(claim): RequireAuth,
    State(pool): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<FeelingTrackerGet>, AppError> {

    let result  = FeelingTrackerService::by_id(&pool.db, id).await?;

    if claim.sub != result.user_id { //We block even admins from consulting these
        return Err(AppError::Unauthorized)
    }

    Ok(Json(result))
}


#[utoipa::path(
    post,
    path = "/feeling_tracker",
    tag = "feeling_tracker",
    request_body = FeelingTrackerCreate,
)]
#[debug_handler]
pub async fn create_feeling_tracker(
    RequireAuth(claims): RequireAuth,
    State(pool): State<AppState>,
    Json(body): Json<FeelingTrackerCreate>
) -> Result<Json<i32>, AppError> {

    if claims.sub != body.user_id {
        return Err(AppError::Unauthorized)
    }

    FeelingTrackerService::create(&pool.db, body)
        .await.map(Json)
}


#[utoipa::path(
    put,
    path = "/feeling_tracker",
    tag = "feeling_tracker",
    request_body = FeelingTrackerUpdate,
)]
#[debug_handler]
pub async fn update_feeling_tracker(
    RequireAuth(claims): RequireAuth,
    State(pool): State<AppState>,
    Path(id): Path<i32>,
    Json(body): Json<FeelingTrackerUpdate>
) -> Result<Json<()>, AppError> {
    let resource = FeelingTrackerService::by_id(&pool.db, id).await?;

    assert_owns_resource(&claims, resource.user_id)?;

    FeelingTrackerService::update(&pool.db, id, body)
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
    RequireAuth(claims): RequireAuth,
    State(pool): State<AppState>,
    Json(body): Json<FeelingTrackerSearchParams>
) -> Result<Json<Vec<FeelingTrackerGet>>, AppError> {

    FeelingTrackerService::search(&pool.db, claims.sub, body)
        .await.map(Json)
}

#[utoipa::path(
    delete,
    path = "/feeling_tracker/{id}",
    tag = "feeling_tracker",
)]
#[debug_handler]
pub async fn delete_feeling_tracker(
    RequireAdmin(_claims): RequireAdmin,
    State(pool): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<()>, AppError> {

    FeelingTrackerService::delete(&pool.db, id)
        .await.map(Json)
}