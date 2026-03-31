use crate::modules::feeling::handler::{create_feeling, delete_feeling, get_feeling_by_id, search_feeling, update_feeling};
use crate::AppState;
use axum::routing::{delete, get, post, put};
use axum::Router;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/{id}", get(get_feeling_by_id))
        .route("/", post(create_feeling))
        .route("/search", post(search_feeling))
        .route("/{id}", put(update_feeling))
        .route("/{id}", delete(delete_feeling))
}
