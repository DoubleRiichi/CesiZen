// ============================================================
// Tests d'intégration transversaux + helpers partagés
// ============================================================

pub mod helpers {
    use axum::Router;
    use sqlx::postgres::PgPoolOptions;

    pub async fn build_test_app() -> Router {
        let url = std::env::var("DATABASE_TEST_URL")
            .expect("DATABASE_TEST_URL must be set for HTTP integration tests");

        let pool = PgPoolOptions::new()
            .max_connections(2)
            .connect(&url)
            .await
            .expect("Failed to connect to test DB");

        crate::build_app(pool)
    }
}

#[cfg(test)]
#[cfg(feature = "integration")]
mod e2e {
    use axum::{body::Body, http::{Request, StatusCode}, Router};
    use serde_json::{json, Value};
    use tower::ServiceExt;
    use super::helpers::build_test_app;

    async fn body_json(response: axum::response::Response) -> Value {
        let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        serde_json::from_slice(&bytes).unwrap_or(Value::Null)
    }

    #[tokio::test]
    async fn full_article_lifecycle() {
        let app: Router = build_test_app().await;

        let user_body = json!({
            "username": "e2e_author",
            "password": "password123",
            "email": "e2e_author@test.com",
            "avatar": "https://avatar.com/e2e",
            "age": 28
        });

        let resp = app.clone().oneshot(
            Request::builder()
                .method("POST").uri("/user/")
                .header("Content-Type", "application/json")
                .body(Body::from(user_body.to_string())).unwrap(),
        ).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
        let user: Value = body_json(resp).await;
        let user_id = user["id"].as_i64().unwrap();

        let article_body = json!({
            "author_id": user_id,
            "title": "Article E2E de test suffisamment long",
            "content": "c".repeat(300),
            "visibility": "Public",
            "tags": []
        });

        let resp = app.clone().oneshot(
            Request::builder()
                .method("POST").uri("/article/")
                .header("Content-Type", "application/json")
                .body(Body::from(article_body.to_string())).unwrap(),
        ).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
        let article_id = body_json(resp).await.as_i64().unwrap();

        let resp = app.clone().oneshot(
            Request::builder()
                .method("GET").uri(format!("/article/{}", article_id))
                .body(Body::empty()).unwrap(),
        ).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
        let article: Value = body_json(resp).await;
        assert_eq!(article["author"]["id"].as_i64().unwrap(), user_id);

        // Cleanup
        let _ = app.clone().oneshot(
            Request::builder()
                .method("DELETE").uri(format!("/article/{}", article_id))
                .body(Body::empty()).unwrap(),
        ).await;
        let _ = app.clone().oneshot(
            Request::builder()
                .method("DELETE").uri(format!("/user/{}", user_id))
                .body(Body::empty()).unwrap(),
        ).await;
    }

    #[tokio::test]
    async fn tag_crud_lifecycle() {
        let app: Router = build_test_app().await;

        let resp = app.clone().oneshot(
            Request::builder()
                .method("POST").uri("/tag/")
                .header("Content-Type", "application/json")
                .body(Body::from(r#"{"name":"E2E_Tag_CI"}"#)).unwrap(),
        ).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
        let tag_id = body_json(resp).await["id"].as_i64().unwrap();

        let resp = app.clone().oneshot(
            Request::builder()
                .method("GET").uri(format!("/tag/{}", tag_id))
                .body(Body::empty()).unwrap(),
        ).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);

        let resp = app.clone().oneshot(
            Request::builder()
                .method("GET").uri("/tag/all")
                .body(Body::empty()).unwrap(),
        ).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
        let all: Value = body_json(resp).await;
        assert!(all.as_array().unwrap().iter().any(|t| t["id"].as_i64() == Some(tag_id)));

        let resp = app.clone().oneshot(
            Request::builder()
                .method("DELETE").uri(format!("/tag/{}", tag_id))
                .body(Body::empty()).unwrap(),
        ).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn malformed_json_returns_422() {
        let app: Router = build_test_app().await;

        let resp = app.oneshot(
            Request::builder()
                .method("POST").uri("/user/")
                .header("Content-Type", "application/json")
                .body(Body::from("not json at all")).unwrap(),
        ).await.unwrap();
        assert_eq!(resp.status(), StatusCode::UNPROCESSABLE_ENTITY);
    }
}