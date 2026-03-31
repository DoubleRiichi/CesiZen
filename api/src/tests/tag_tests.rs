// ============================================================
// Tests — Module Tag
// ============================================================

#[cfg(test)]
mod unit {
    use crate::modules::tag::dto::{TagCreate, TagGet};
    use crate::modules::tag::model::TagRow;

    #[test]
    fn tag_get_from_row() {
        let row = TagRow { id: 42, name: "Rust".to_string() };
        let dto = TagGet::from(row);
        assert_eq!(dto.id, 42);
        assert_eq!(dto.name, "Rust");
    }

    #[test]
    fn tag_create_deserializes() {
        let json = r#"{"name":"PostgreSQL"}"#;
        let dto: TagCreate = serde_json::from_str(json).unwrap();
        assert_eq!(dto.name, "PostgreSQL");
    }

    #[test]
    fn tag_get_serializes() {
        let dto = TagGet { id: 1, name: "Axum".to_string() };
        let json = serde_json::to_string(&dto).unwrap();
        assert!(json.contains("\"id\":1"));
        assert!(json.contains("\"name\":\"Axum\""));
    }
}

// ── Tests d'intégration ───────────────────────────────────────

#[cfg(test)]
#[cfg(feature = "integration")]
mod integration {
    use crate::errors::app::AppError;
    use crate::modules::tag::dto::TagCreate;
    use crate::modules::tag::service::TagService;
    use sqlx::PgPool;

    async fn get_test_pool() -> PgPool {
        let url = std::env::var("DATABASE_TEST_URL").unwrap();
        PgPool::connect(&url).await.unwrap()
    }

    #[tokio::test]
    async fn create_tag_returns_tag_get() {
        let pool = get_test_pool().await;

        let input = TagCreate { name: "TestTagUnique_CI".to_string() };
        let result = TagService::create(&pool, input).await;
        assert!(result.is_ok(), "{:?}", result.err());

        let tag = result.unwrap();
        assert_eq!(tag.name, "TestTagUnique_CI");
        assert!(tag.id > 0);

        let _ = TagService::delete(&pool, tag.id).await;
    }

    #[tokio::test]
    async fn fetch_tag_by_id() {
        let pool = get_test_pool().await;

        let created = TagService::create(&pool, TagCreate { name: "TagById_CI".to_string() })
            .await
            .unwrap();

        let found = TagService::by_id(&pool, created.id).await;
        assert!(found.is_ok());
        assert_eq!(found.unwrap().name, "TagById_CI");

        let _ = TagService::delete(&pool, created.id).await;
    }

    #[tokio::test]
    async fn fetch_nonexistent_tag_returns_not_found() {
        let pool = get_test_pool().await;
        let result = TagService::by_id(&pool, 999_999).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn delete_tag() {
        let pool = get_test_pool().await;
        let tag = TagService::create(&pool, TagCreate { name: "TagToDelete_CI".to_string() })
            .await
            .unwrap();

        let del = TagService::delete(&pool, tag.id).await;
        assert!(del.is_ok());

        let result = TagService::by_id(&pool, tag.id).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn find_all_tags_returns_list() {
        let pool = get_test_pool().await;

        let tag = TagService::create(&pool, TagCreate { name: "TagAll_CI".to_string() })
            .await
            .unwrap();

        let all = TagService::find_all(&pool).await.unwrap();
        assert!(!all.is_empty());
        assert!(all.iter().any(|t| t.id == tag.id));

        let _ = TagService::delete(&pool, tag.id).await;
    }

    #[tokio::test]
    async fn create_duplicate_tag_returns_error() {
        let pool = get_test_pool().await;
        let name = "DuplicateTag_CI";

        let first = TagService::create(&pool, TagCreate { name: name.to_string() })
            .await
            .unwrap();

        // UNIQUE constraint should trigger
        let second = TagService::create(&pool, TagCreate { name: name.to_string() }).await;
        assert!(second.is_err());

        let _ = TagService::delete(&pool, first.id).await;
    }
}