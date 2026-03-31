use crate::modules::user::handler::{create_user, delete_user, get_user_by_id, login, search_user};
use crate::AppState;
use axum::routing::{delete, get, post};
use axum::Router;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/{id}", get(get_user_by_id))
        .route("/", post(create_user))
        .route("/search", post(search_user))
        .route("/{id}", delete(delete_user))
        .route("/login", post(login))
}
