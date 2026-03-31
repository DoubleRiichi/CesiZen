use crate::modules::feeling_tracker::handler::{create_feeling_tracker, delete_feeling_tracker, get_feeling_tracker_by_id, search_feeling_tracker};
use crate::AppState;
use axum::routing::{delete, get, post};
use axum::Router;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/{id}", get(get_feeling_tracker_by_id))
        .route("/", post(create_feeling_tracker))
        .route("/search", post(search_feeling_tracker))
        .route("/{id}", delete(delete_feeling_tracker))
}
