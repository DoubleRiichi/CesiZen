pub mod user_tests;
pub mod article_tests;
pub mod tag_tests;
pub mod feeling_tests;
pub mod feeling_tracker_tests;
pub mod integration_tests;

use crate::AppState;
use crate::modules::{feeling, feeling_category, feeling_tracker};

pub fn build_app(pool: sqlx::PgPool) -> axum::Router {
    use crate::modules::{article, tag, user};

    let state = AppState { db: pool };
    axum::Router::new()
        .nest("/article/", article::router::router())
        .nest("/user/", user::router::router())
        .nest("/tag/", tag::router::router())
        .nest("/feeling/", feeling::router::router())
        .nest("/feeling_category", feeling_category::router::router())
        .nest("/feeling_tracker", feeling_tracker::router::router())
        .with_state(state)
}