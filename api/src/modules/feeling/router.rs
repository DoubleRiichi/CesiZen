use axum::Router;
use axum::routing::{delete, get, post};
use crate::AppState;
use crate::modules::feeling::handler::{create_feeling, delete_feeling, get_feeling_by_id, search_feeling};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/{id}", get(get_feeling_by_id))
        .route("/", post(create_feeling))
        .route("/search", post(search_feeling))
        .route("/", delete(delete_feeling))
}
