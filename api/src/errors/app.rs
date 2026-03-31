use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error")]
    Database(String),

    #[error("{0} not found")]
    NotFound(String),

    #[error("Bad request : {0}")]
    Validation(String),

    #[error("Internal server error")]
    Internal(String),

    #[error("Conflict on {0}")]
    Conflict(String),
}

impl From<validator::ValidationErrors> for AppError {
    fn from(err: validator::ValidationErrors) -> Self {
        AppError::Validation(err.to_string())
    }
}


impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {

        match err {
            sqlx::Error::RowNotFound => AppError::NotFound(String::from("Not found")),
            other => {
                eprintln!("SQLx error : {:?}", other);
                AppError::Internal("Internal server error".to_string())
            }
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, msg) = match self {
            AppError::Database(e) => (StatusCode::INTERNAL_SERVER_ERROR, e),
            AppError::NotFound(e) => (StatusCode::NOT_FOUND, e),
            AppError::Validation(e) => (StatusCode::BAD_REQUEST, e),
            AppError::Internal(e) => (StatusCode::INTERNAL_SERVER_ERROR, e),
            AppError::Conflict(e) => (StatusCode::CONFLICT, e),

        };

        (
            status,
            Json(serde_json::json!({
                "error": msg,
                "status": status.as_u16(),
            })),
        ).into_response()


    }
}

/*
Exemple implementation for error results
async fn get_user(
    Path(id): Path<Uuid>, State(pool)) -> Result<Json<User>, AppError> {
    let user = sqlx:query_as::<_, User>(
        "Select * from users where id = $1"
        ).bind(id)
        .fetch_optional(&pool)
        .await?
        .ok_or(AppError::NotFound)?;

        Ok(Json(user))
}
 */