// ============================================================
// Tests d'intégration transversaux + helpers partagés
// ============================================================
//
// Ce fichier contient :
//   1. build_test_app()  — construit l'application Axum avec une pool de test
//   2. Tests end-to-end qui traversent plusieurs modules
// ============================================================
pub mod helpers {
    use axum::Router;
    use sqlx::{PgPool, postgres::PgPoolOptions};

    /// Construit une instance de l'application Axum connectée à DATABASE_TEST_URL.
    /// Utilisée par les tests HTTP via `tower::ServiceExt::oneshot`.
    pub async fn build_test_app() -> Router {
        let url = std::env::var("DATABASE_TEST_URL")
            .expect("DATABASE_TEST_URL must be set for HTTP integration tests");

        let pool = PgPoolOptions::new()
            .max_connections(2)
            .connect(&url)
            .await
            .expect("Failed to connect to test DB");

        crate::tests::build_app(pool)
    }
}

// ── Tests end-to-end transversaux ────────────────────────────

#[cfg(test)]
#[cfg(feature = "integration")]
mod e2e {
    use axum::{body::Body, http::{Request, StatusCode}};
    use serde_json::{json, Value};
    use tower::ServiceExt;
    use crate::helpers::build_test_app;

    // Utilitaire : désérialise le body d'une réponse
    async fn body_json(response: axum::response::Response) -> Value {
        let bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        serde_json::from_slice(&bytes).unwrap_or(Value::Null)
    }

    // ── Scénario complet : créer un user, un article, le rechercher ──

    #[tokio::test]
    async fn full_article_lifecycle() {
        let app: axum::Router = build_test_app().await;

        // 1. Créer un utilisateur
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
                .body(Body::from(user_body.to_string())).unwrap()
        ).await.unwrap();

        assert_eq!(resp.status(), StatusCode::OK);
        let user: Value = body_json(resp).await;
        let user_id = user["id"].as_i64().unwrap();

        // 2. Créer un article avec cet auteur
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
                .body(Body::from(article_body.to_string())).unwrap()
        ).await.unwrap();

        assert_eq!(resp.status(), StatusCode::OK);
        let article_id: Value = body_json(resp).await;
        let article_id = article_id.as_i64().unwrap();

        // 3. Récupérer l'article par ID
        let resp = app.clone().oneshot(
            Request::builder()
                .method("GET").uri(format!("/article/{}", article_id))
                .body(Body::empty()).unwrap()
        ).await.unwrap();

        assert_eq!(resp.status(), StatusCode::OK);
        let article: Value = body_json(resp).await;
        assert_eq!(article["id"].as_i64().unwrap(), article_id);
        assert_eq!(article["author"]["id"].as_i64().unwrap(), user_id);

        // 4. Rechercher l'article
        let search_body = json!({
            "author_id": user_id,
            "page_size": 10
        });

        let resp = app.clone().oneshot(
            Request::builder()
                .method("POST").uri("/article/search")
                .header("Content-Type", "application/json")
                .body(Body::from(search_body.to_string())).unwrap()
        ).await.unwrap();

        assert_eq!(resp.status(), StatusCode::OK);
        let articles: Value = body_json(resp).await;
        let arr = articles.as_array().unwrap();
        assert!(arr.iter().any(|a| a["id"].as_i64() == Some(article_id)));

        // 5. Supprimer l'article puis l'utilisateur
        let resp = app.clone().oneshot(
            Request::builder()
                .method("DELETE").uri(format!("/article/{}", article_id))
                .body(Body::empty()).unwrap()
        ).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);

        let resp = app.clone().oneshot(
            Request::builder()
                .method("DELETE").uri(format!("/user/{}", user_id))
                .body(Body::empty()).unwrap()
        ).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }

    // ── Scénario : tags complets ──────────────────────────────

    #[tokio::test]
    async fn tag_crud_lifecycle() {
        let app = build_test_app().await;

        // Create
        let resp = app.clone().oneshot(
            Request::builder()
                .method("POST").uri("/tag/")
                .header("Content-Type", "application/json")
                .body(Body::from(r#"{"name":"E2E_Tag_CI"}"#)).unwrap()
        ).await.unwrap();

        assert_eq!(resp.status(), StatusCode::OK);
        let tag: Value = body_json(resp).await;
        let tag_id = tag["id"].as_i64().unwrap();

        // Get by ID
        let resp = app.clone().oneshot(
            Request::builder()
                .method("GET").uri(format!("/tag/{}", tag_id))
                .body(Body::empty()).unwrap()
        ).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);

        // List all — should contain the created tag
        let resp = app.clone().oneshot(
            Request::builder()
                .method("GET").uri("/tag/all")
                .body(Body::empty()).unwrap()
        ).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
        let all: Value = body_json(resp).await;
        let arr = all.as_array().unwrap();
        assert!(arr.iter().any(|t| t["id"].as_i64() == Some(tag_id)));

        // Delete
        let resp = app.clone().oneshot(
            Request::builder()
                .method("DELETE").uri(format!("/tag/{}", tag_id))
                .body(Body::empty()).unwrap()
        ).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }

    // ── Erreurs communes ─────────────────────────────────────

    #[tokio::test]
    async fn malformed_json_returns_422() {
        let app = build_test_app().await;

        let resp = app.oneshot(
            Request::builder()
                .method("POST").uri("/user/")
                .header("Content-Type", "application/json")
                .body(Body::from("not json at all")).unwrap()
        ).await.unwrap();

        // Axum renvoie 422 Unprocessable Entity pour JSON invalide
        assert_eq!(resp.status(), StatusCode::UNPROCESSABLE_ENTITY);
    }
}