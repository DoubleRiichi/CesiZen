use crate::modules::tag::handler::{all_tag, create_tag, delete_tag, get_tag_by_id};
use crate::AppState;
use axum::routing::{delete, get, post};
use axum::Router;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/{id}", get(get_tag_by_id))
        .route("/", post(create_tag))
        .route("/all", get(all_tag))
        .route("/{id}", delete(delete_tag))
}
