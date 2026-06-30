pub mod modules;
pub mod errors;
pub mod docs;
pub mod auth;
#[cfg(test)]
mod tests;

use dotenv::dotenv;
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::env;
use tokio::net::TcpListener;
use tower_http::cors::{CorsLayer, Any};
use axum::http::Method;
#[cfg(feature = "swagger")] 
use utoipa_swagger_ui::SwaggerUi;
#[cfg(feature = "swagger")] 
use utoipa::OpenApi;
#[cfg(feature = "swagger")] 
use crate::docs::ApiDoc;



pub fn build_app(pool: sqlx::PgPool) -> axum::Router {
    use crate::modules::{article, feeling, feeling_category, feeling_tracker, tag, user};

    let state = crate::AppState { db: pool };

    let app = axum::Router::new()
        .nest("/article", article::router::router())
        .nest("/user", user::router::router())
        .nest("/tag", tag::router::router())
        .nest("/feeling", feeling::router::router())
        .nest("/feeling_category", feeling_category::router::router())
        .nest("/feeling_tracker", feeling_tracker::router::router())
        .with_state(state);
   
    #[cfg(feature = "swagger")]
    app.merge(
            SwaggerUi::new("/swagger-ui")
                .url("/cesizen_api-doc/openapi.json", ApiDoc::openapi()),
        );
    
    app
}

#[derive(Clone)]
pub struct AppState {
    db: PgPool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env");

    println!("Attempting to reach the database...");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("Failed to connect to PostgreSQL");

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers(Any);

    let app = build_app(pool).layer(cors);



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