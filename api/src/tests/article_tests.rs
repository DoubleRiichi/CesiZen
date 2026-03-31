// ============================================================
// Tests — Module Article
// ============================================================

#[cfg(test)]
mod unit {
    use validator::Validate;
    use crate::modules::article::dto::{ArticleCreate, ArticleUpdate};

    fn valid_article_create() -> ArticleCreate {
        ArticleCreate {
            author_id: 1,
            title: "Un titre suffisamment long pour être valide".to_string(),
            content: "x".repeat(300),
            visibility: "Public".to_string(),
            tags: vec![1, 2],
        }
    }

    #[test]
    fn article_create_valid() {
        assert!(valid_article_create().validate().is_ok());
    }

    #[test]
    fn article_create_title_too_short() {
        let a = ArticleCreate {
            title: "Court".to_string(), // < 10 chars
            ..valid_article_create()
        };
        let err = a.validate().unwrap_err().to_string();
        assert!(err.contains("title"));
    }

    #[test]
    fn article_create_title_too_long() {
        let a = ArticleCreate {
            title: "t".repeat(257),
            ..valid_article_create()
        };
        assert!(a.validate().is_err());
    }

    #[test]
    fn article_create_content_too_short() {
        let a = ArticleCreate {
            content: "Trop court.".to_string(), // < 300 chars
            ..valid_article_create()
        };
        let err = a.validate().unwrap_err().to_string();
        assert!(err.contains("content"));
    }

    #[test]
    fn article_create_content_exact_min_length() {
        let a = ArticleCreate {
            content: "c".repeat(300),
            ..valid_article_create()
        };
        assert!(a.validate().is_ok());
    }

    #[test]
    fn article_create_empty_tags_allowed() {
        let a = ArticleCreate {
            tags: vec![],
            ..valid_article_create()
        };
        assert!(a.validate().is_ok());
    }

    // ── ArticleUpdate validations ─────────────────────────────

    #[test]
    fn article_update_all_none_is_valid() {
        let u = ArticleUpdate {
            author_id: None,
            title: None,
            content: None,
            is_deleted: None,
            visibility: None,
            tags: None,
        };
        assert!(u.validate().is_ok());
    }

    #[test]
    fn article_update_invalid_title_too_short() {
        let u = ArticleUpdate {
            title: Some("Hi".to_string()), // < 10 chars
            author_id: None, content: None,
            is_deleted: None, visibility: None, tags: None,
        };
        assert!(u.validate().is_err());
    }

    #[test]
    fn article_update_valid_title() {
        let u = ArticleUpdate {
            title: Some("Titre valide long enough".to_string()),
            author_id: None, content: None,
            is_deleted: None, visibility: None, tags: None,
        };
        assert!(u.validate().is_ok());
    }
}

// ── Tests d'intégration ───────────────────────────────────────

#[cfg(test)]
#[cfg(feature = "integration")]
mod integration {
    use sqlx::PgPool;
    use crate::modules::article::dto::{ArticleCreate, ArticleSearchParams, ArticleUpdate};
    use crate::modules::article::service::ArticleService;
    use crate::errors::app::AppError;

    async fn get_test_pool() -> PgPool {
        let url = std::env::var("DATABASE_TEST_URL").unwrap();
        PgPool::connect(&url).await.unwrap()
    }

    async fn create_test_article(pool: &PgPool) -> i32 {
        let input = ArticleCreate {
            author_id: 1,
            title: "Test Article pour integration".to_string(),
            content: "c".repeat(300),
            visibility: "Public".to_string(),
            tags: vec![],
        };
        ArticleService::create(pool, input).await.expect("Failed to create test article")
    }

    #[tokio::test]
    async fn create_article_returns_id() {
        let pool = get_test_pool().await;
        let id = create_test_article(&pool).await;
        assert!(id > 0);
        // Cleanup
        let _ = ArticleService::delete(&pool, id).await;
    }

    #[tokio::test]
    async fn fetch_article_by_id_found() {
        let pool = get_test_pool().await;
        let id = create_test_article(&pool).await;

        let result = ArticleService::by_id(&pool, id).await;
        assert!(result.is_ok(), "{:?}", result.err());
        let article = result.unwrap();
        assert_eq!(article.id, id);

        let _ = ArticleService::delete(&pool, id).await;
    }

    #[tokio::test]
    async fn fetch_article_not_found() {
        let pool = get_test_pool().await;
        let result = ArticleService::by_id(&pool, 999_999).await;
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), AppError::NotFound(_)));
    }

    #[tokio::test]
    async fn update_article_title() {
        let pool = get_test_pool().await;
        let id = create_test_article(&pool).await;

        let update = ArticleUpdate {
            title: Some("Nouveau titre suffisamment long".to_string()),
            author_id: None, content: None,
            is_deleted: None, visibility: None, tags: None,
        };
        let result = ArticleService::update(&pool, id, update).await;
        assert!(result.is_ok());

        let article = ArticleService::by_id(&pool, id).await.unwrap();
        assert_eq!(article.title, "Nouveau titre suffisamment long");

        let _ = ArticleService::delete(&pool, id).await;
    }

    #[tokio::test]
    async fn delete_article() {
        let pool = get_test_pool().await;
        let id = create_test_article(&pool).await;

        let del = ArticleService::delete(&pool, id).await;
        assert!(del.is_ok());

        let result = ArticleService::by_id(&pool, id).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn search_articles_by_author() {
        let pool = get_test_pool().await;
        let id = create_test_article(&pool).await;

        let params = ArticleSearchParams {
            author_id: Some(1),
            title: None, content: None,
            start_date: None, end_date: None,
            tag_ids: None, cursor: None,
            page_size: Some(10),
        };

        let result = ArticleService::search(&pool, params).await;
        assert!(result.is_ok());
        let articles = result.unwrap();
        assert!(!articles.is_empty());
        for a in &articles {
            assert_eq!(a.author.id, 1);
        }

        let _ = ArticleService::delete(&pool, id).await;
    }

    #[tokio::test]
    async fn search_articles_title_filter() {
        let pool = get_test_pool().await;
        let id = create_test_article(&pool).await;

        let params = ArticleSearchParams {
            title: Some("integration".to_string()),
            author_id: None, content: None,
            start_date: None, end_date: None,
            tag_ids: None, cursor: None, page_size: Some(10),
        };

        let result = ArticleService::search(&pool, params).await.unwrap();
        assert!(result.iter().any(|a| a.id == id));

        let _ = ArticleService::delete(&pool, id).await;
    }

    #[tokio::test]
    async fn search_page_size_too_high() {
        let pool = get_test_pool().await;
        let params = ArticleSearchParams {
            page_size: Some(501),
            author_id: None, title: None, content: None,
            start_date: None, end_date: None,
            tag_ids: None, cursor: None,
        };
        let result = ArticleService::search(&pool, params).await;
        assert!(matches!(result.unwrap_err(), AppError::Validation(_)));
    }
}