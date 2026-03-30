pub mod modules;
pub mod errors;
pub mod docs;
pub mod auth;
#[cfg(test)]
mod tests;

use axum::{
    Router,
    routing::get,
};
use sqlx::{PgPool, postgres::PgPoolOptions};
use std::env;
use axum::routing::post;
use tokio::net::TcpListener;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use dotenv::dotenv;

use crate::docs::ApiDoc;
use crate::modules::{article, feeling, feeling_category, feeling_tracker, tag, user};
use crate::modules::article::handler::{create_article, get_article_by_id, search_article};

#[derive(Clone)]
struct AppState {
    db: PgPool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("Failed to connect to PostgreSQL");

    // State can be PgPool directly (it's already Clone + Send + Sync)
    // or you can wrap it in Arc<AppState> if you prefer
    let state = AppState { db: pool };

    let app = Router::new()
        .nest("/article", article::router::router())
        .nest("/user", user::router::router())
        .nest("/tag", tag::router::router())
        .nest("/feeling", feeling::router::router())
        .nest("/feeling_category", feeling_category::router::router())
        .nest("/feeling_tracker", feeling_tracker::router::router())
        .merge(
            SwaggerUi::new("/swagger-ui")
                .url("/api-doc/openapi.json", ApiDoc::openapi()),
        )
        .with_state(state);

    let listener = TcpListener::bind("0.0.0.0:8080").await
        .expect("Failed to bind 0.0.0.0:8080");

    println!("Server running at http://0.0.0.0:8080");
    println!("Swagger UI:     http://0.0.0.0:8080/swagger-ui");

    // This is the correct way in axum 0.7+
    axum::serve(listener, app)
        .await
        .expect("Server failed");

    Ok(())
}