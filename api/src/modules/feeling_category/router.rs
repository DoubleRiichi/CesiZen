use crate::modules::feeling_category::handler::{create_feeling_category, delete_feeling_category, get_feeling_category_by_id, search_feeling_category};
use crate::AppState;
use axum::routing::{delete, get, post};
use axum::Router;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/{id}", get(get_feeling_category_by_id))
        .route("/", post(create_feeling_category))
        .route("/search", post(search_feeling_category))
        .route("/{id}", delete(delete_feeling_category))
}
