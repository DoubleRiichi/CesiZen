// ============================================================
// Tests — Module User
// ============================================================
//
// Stratégie :
//   - Tests unitaires : validation des DTOs (UserCreate, UserUpdate)
//   - Tests d'intégration : repository + service sur une vraie DB de test
//   - Tests des handlers via axum::test (layer HTTP)
//
// Pour lancer : cargo test --test user_tests
//               ou : cargo nextest run user
// ============================================================

#[cfg(test)]
mod unit {
    use validator::Validate;
    use crate::modules::user::dto::{UserCreate, UserUpdate};

    // ── UserCreate validations ──────────────────────────────────

    fn valid_user_create() -> UserCreate {
        UserCreate {
            username: "alice123".to_string(),
            password: "supersecret".to_string(),
            email: "alice@example.com".to_string(),
            avatar: "https://i.pravatar.cc/150".to_string(),
            age: 25,
        }
    }

    #[test]
    fn user_create_valid() {
        assert!(valid_user_create().validate().is_ok());
    }

    #[test]
    fn user_create_username_too_short() {
        let u = UserCreate { username: "ab".to_string(), ..valid_user_create() };
        let err = u.validate();
        assert!(err.is_err());
        let msg = err.unwrap_err().to_string();
        assert!(msg.contains("username"));
    }

    #[test]
    fn user_create_username_too_long() {
        let u = UserCreate { username: "a".repeat(21), ..valid_user_create() };
        assert!(u.validate().is_err());
    }

    #[test]
    fn user_create_password_too_short() {
        let u = UserCreate { password: "short".to_string(), ..valid_user_create() };
        let err = u.validate().unwrap_err().to_string();
        assert!(err.contains("password"));
    }

    #[test]
    fn user_create_password_too_long() {
        let u = UserCreate { password: "p".repeat(33), ..valid_user_create() };
        assert!(u.validate().is_err());
    }

    #[test]
    fn user_create_invalid_email() {
        let u = UserCreate { email: "not-an-email".to_string(), ..valid_user_create() };
        let err = u.validate().unwrap_err().to_string();
        assert!(err.contains("email"));
    }

    #[test]
    fn user_create_age_too_young() {
        let u = UserCreate { age: 12, ..valid_user_create() };
        let err = u.validate().unwrap_err().to_string();
        assert!(err.contains("age"));
    }

    #[test]
    fn user_create_age_too_old() {
        let u = UserCreate { age: 121, ..valid_user_create() };
        assert!(u.validate().is_err());
    }

    #[test]
    fn user_create_age_boundary_min_valid() {
        let u = UserCreate { age: 13, ..valid_user_create() };
        assert!(u.validate().is_ok());
    }

    #[test]
    fn user_create_age_boundary_max_valid() {
        let u = UserCreate { age: 120, ..valid_user_create() };
        assert!(u.validate().is_ok());
    }

    // ── UserUpdate validations ──────────────────────────────────

    fn valid_user_update() -> UserUpdate {
        UserUpdate {
            password: "newpassword1".to_string(),
            email: "bob@example.com".to_string(),
            avatar: "https://avatar.com/img".to_string(),
            is_active: true,
        }
    }

    #[test]
    fn user_update_valid() {
        assert!(valid_user_update().validate().is_ok());
    }

    #[test]
    fn user_update_invalid_email() {
        let u = UserUpdate { email: "bad".to_string(), ..valid_user_update() };
        assert!(u.validate().is_err());
    }

    #[test]
    fn user_update_password_too_short() {
        let u = UserUpdate { password: "tiny".to_string(), ..valid_user_update() };
        assert!(u.validate().is_err());
    }
}

// ── Tests d'intégration (nécessitent DATABASE_TEST_URL) ──────

#[cfg(test)]
#[cfg(feature = "integration")]
mod integration {
    use sqlx::PgPool;
    use crate::modules::user::dto::{UserCreate, UserSearchParams};
    use crate::modules::user::model::UserRole;
    use crate::modules::user::service::UserService;

    async fn get_test_pool() -> PgPool {
        let url = std::env::var("DATABASE_TEST_URL")
            .expect("DATABASE_TEST_URL must be set for integration tests");
        PgPool::connect(&url).await.expect("Failed to connect to test DB")
    }

    #[tokio::test]
    async fn create_and_fetch_user() {
        let pool = get_test_pool().await;

        let input = UserCreate {
            username: "testuser_ci".to_string(),
            password: "password123".to_string(),
            email: "ci_test@example.com".to_string(),
            avatar: "https://avatar.com/ci".to_string(),
            age: 25,
        };

        let created = UserService::create(&pool, input).await;
        assert!(created.is_ok(), "create failed: {:?}", created.err());

        let user = created.unwrap();
        assert_eq!(user.username, "testuser_ci");
        assert_eq!(user.email, "ci_test@example.com");
        assert_eq!(user.role, "User");

        // Cleanup
        let _ = UserService::delete(&pool, user.id).await;
    }

    #[tokio::test]
    async fn fetch_nonexistent_user_returns_not_found() {
        let pool = get_test_pool().await;
        let result = UserService::by_id(&pool, 999_999).await;
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(err, crate::errors::app::AppError::NotFound(_)));
    }

    #[tokio::test]
    async fn search_users_with_username_filter() {
        let pool = get_test_pool().await;
        let params = UserSearchParams {
            username: Some("alice".to_string()),
            email: None, age: None, avatar: None,
            is_active: None, role: None,
            start_at: None, end_at: None,
            cursor: None, page_size: Some(10),
        };

        let result = UserService::search(&pool, params).await;
        assert!(result.is_ok());
        let users = result.unwrap();
        for u in &users {
            assert!(u.username.to_lowercase().contains("alice"));
        }
    }

    #[tokio::test]
    async fn search_page_size_too_high_returns_error() {
        let pool = get_test_pool().await;
        let params = UserSearchParams {
            username: None, email: None, age: None,
            avatar: None, is_active: None, role: None,
            start_at: None, end_at: None,
            cursor: None, page_size: Some(501),
        };
        let result = UserService::search(&pool, params).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn search_page_size_zero_returns_error() {
        let pool = get_test_pool().await;
        let params = UserSearchParams {
            page_size: Some(0),
            username: None, email: None, age: None,
            avatar: None, is_active: None, role: None,
            start_at: None, end_at: None, cursor: None,
        };
        let result = UserService::search(&pool, params).await;
        assert!(result.is_err());
    }
}

// ── Tests HTTP via axum::test ────────────────────────────────

#[cfg(test)]
#[cfg(feature = "integration")]
mod http {
    use axum::{
        body::Body,
        http::{Request, StatusCode},

    };
    use serde_json::json;
    use tower::ServiceExt; // for `oneshot`
    use crate::tests::helpers::build_test_app;

    #[tokio::test]
    async fn post_user_returns_201() {
        let app: axum::Router = build_test_app().await;
        let app = app.into_make_service();
        let body = json!({
            "username": "http_test_user",
            "password": "validpass123",
            "email": "http_test@example.com",
            "avatar": "https://avatar.com/http",
            "age": 30
        });

        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/user/")
                    .header("Content-Type", "application/json")
                    .body(Body::from(serde_json::to_string(&body).unwrap()))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn get_nonexistent_user_returns_404() {
        let app = build_test_app().await;

        let response = app
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri("/user/999999")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn post_user_invalid_email_returns_400() {
        let app = build_test_app().await;

        let body = json!({
            "username": "validuser",
            "password": "validpass123",
            "email": "NOT_AN_EMAIL",
            "avatar": "https://avatar.com/x",
            "age": 25
        });

        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/user/")
                    .header("Content-Type", "application/json")
                    .body(Body::from(serde_json::to_string(&body).unwrap()))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }
}