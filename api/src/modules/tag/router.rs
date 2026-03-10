use axum::Router;
use axum::routing::{delete, get, post};
use crate::AppState;
use crate::modules::tag::handler::{all_tag, create_tag, delete_tag, get_tag_by_id};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/{id}", get(get_tag_by_id))
        .route("/", post(create_tag))
        .route("/all", get(all_tag))
        .route("/", delete(delete_tag))
}
