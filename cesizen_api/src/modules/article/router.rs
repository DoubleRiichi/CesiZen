use crate::modules::article::handler::{create_article, delete_article, get_article_by_id, search_article};
use crate::AppState;
use axum::routing::{delete, get, post};
use axum::Router;


pub fn router() -> Router<AppState> {
    Router::new()
        .route("/{id}", get(get_article_by_id))
        .route("/", post(create_article))
        .route("/search", post(search_article))
        .route("/{id}", delete(delete_article))
}
