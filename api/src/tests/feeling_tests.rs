// ============================================================
// Tests — Module FeelingCategory
// ============================================================

#[cfg(test)]
mod feeling_category_unit {
    use crate::modules::feeling_category::dto::{FeelingCategoryCreate, FeelingCategoryGet, FeelingCategoryUpdate};
    use crate::modules::feeling_category::model::FeelingCategoryRow;
    use chrono::{DateTime, Utc};

    fn fake_now() -> DateTime<Utc> {
        chrono::DateTime::from_timestamp(0, 0).unwrap()
    }

    #[test]
    fn feeling_category_get_from_row() {
        let row = FeelingCategoryRow {
            id: 1,
            name: "Positive".to_string(),
            created_at: fake_now(),
            updated_at: fake_now(),
        };
        let dto = FeelingCategoryGet::from(row);
        assert_eq!(dto.id, 1);
        assert_eq!(dto.name, "Positive");
    }

    #[test]
    fn feeling_category_create_deserializes() {
        let json = r#"{"name":"Anxieux"}"#;
        let dto: FeelingCategoryCreate = serde_json::from_str(json).unwrap();
        assert_eq!(dto.name, "Anxieux");
    }

    #[test]
    fn feeling_category_update_deserializes() {
        let json = r#"{"name":"Calme"}"#;
        let dto: FeelingCategoryUpdate = serde_json::from_str(json).unwrap();
        assert_eq!(dto.name, "Calme");
    }
}

#[cfg(test)]
#[cfg(feature = "integration")]
mod feeling_category_integration {
    use sqlx::PgPool;
    use crate::modules::feeling_category::dto::{FeelingCategoryCreate, FeelingCategorySearchParams, FeelingCategoryUpdate};
    use crate::modules::feeling_category::repository::FeelingCategoryRepository;

    async fn get_test_pool() -> PgPool {
        let url = std::env::var("DATABASE_TEST_URL").unwrap();
        PgPool::connect(&url).await.unwrap()
    }

    #[tokio::test]
    async fn create_and_fetch_feeling_category() {
        let pool = get_test_pool().await;

        let id = FeelingCategoryRepository::create(
            &pool,
            FeelingCategoryCreate { name: "CI_Positif".to_string() },
        )
            .await
            .unwrap();

        assert!(id > 0);

        let row = FeelingCategoryRepository::by_id(&pool, id).await.unwrap();
        assert_eq!(row.name, "CI_Positif");

        let _ = FeelingCategoryRepository::delete(&pool, id).await;
    }

    #[tokio::test]
    async fn update_feeling_category_name() {
        let pool = get_test_pool().await;

        let id = FeelingCategoryRepository::create(
            &pool,
            FeelingCategoryCreate { name: "CI_ToUpdate".to_string() },
        )
            .await
            .unwrap();

        FeelingCategoryRepository::update(
            &pool,
            id,
            FeelingCategoryUpdate { name: "CI_Updated".to_string() },
        )
            .await
            .unwrap();

        let row = FeelingCategoryRepository::by_id(&pool, id).await.unwrap();
        assert_eq!(row.name, "CI_Updated");

        let _ = FeelingCategoryRepository::delete(&pool, id).await;
    }

    #[tokio::test]
    async fn delete_feeling_category() {
        let pool = get_test_pool().await;

        let id = FeelingCategoryRepository::create(
            &pool,
            FeelingCategoryCreate { name: "CI_ToDelete".to_string() },
        )
            .await
            .unwrap();

        FeelingCategoryRepository::delete(&pool, id).await.unwrap();
        let result = FeelingCategoryRepository::by_id(&pool, id).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn search_feeling_category_by_name() {
        let pool = get_test_pool().await;

        let id = FeelingCategoryRepository::create(
            &pool,
            FeelingCategoryCreate { name: "CI_SearchTarget".to_string() },
        )
            .await
            .unwrap();

        let params = FeelingCategorySearchParams {
            name: Some("SearchTarget".to_string()),
            start_at: None, end_at: None,
            cursor: None, page_size: Some(10),
        };

        let results = FeelingCategoryRepository::search(&pool, params, 10).await.unwrap();
        assert!(results.iter().any(|r| r.id == id));

        let _ = FeelingCategoryRepository::delete(&pool, id).await;
    }
}

// ============================================================
// Tests — Module Feeling
// ============================================================

#[cfg(test)]
mod feeling_unit {
    use crate::modules::feeling::dto::{FeelingCreate, FeelingUpdate, FeelingSearchParams};

    #[test]
    fn feeling_create_deserializes() {
        let json = r#"{"feeling_category_id":1,"name":"Heureux"}"#;
        let dto: FeelingCreate = serde_json::from_str(json).unwrap();
        assert_eq!(dto.feeling_category_id, 1);
        assert_eq!(dto.name, "Heureux");
    }

    #[test]
    fn feeling_update_deserializes() {
        let json = r#"{"feeling_category_id":2,"name":"Triste"}"#;
        let dto: FeelingUpdate = serde_json::from_str(json).unwrap();
        assert_eq!(dto.feeling_category_id, 2);
        assert_eq!(dto.name, "Triste");
    }

    #[test]
    fn feeling_search_params_all_optional() {
        let json = r#"{}"#;
        let dto: FeelingSearchParams = serde_json::from_str(json).unwrap();
        assert!(dto.name.is_none());
        assert!(dto.feeling_category_id.is_none());
        assert!(dto.page_size.is_none());
    }

    #[test]
    fn feeling_search_params_partial() {
        let json = r#"{"feeling_category_id":1,"page_size":20}"#;
        let dto: FeelingSearchParams = serde_json::from_str(json).unwrap();
        assert_eq!(dto.feeling_category_id, Some(1));
        assert_eq!(dto.page_size, Some(20));
    }
}

#[cfg(test)]
#[cfg(feature = "integration")]
mod feeling_integration {
    use sqlx::PgPool;
    use crate::modules::feeling::dto::{FeelingCreate, FeelingSearchParams, FeelingUpdate};
    use crate::modules::feeling::repository::FeelingRepository;
    use crate::modules::feeling_category::dto::FeelingCategoryCreate;
    use crate::modules::feeling_category::repository::FeelingCategoryRepository;

    async fn get_test_pool() -> PgPool {
        let url = std::env::var("DATABASE_TEST_URL").unwrap();
        PgPool::connect(&url).await.unwrap()
    }

    async fn setup_category(pool: &PgPool) -> i32 {
        FeelingCategoryRepository::create(
            pool,
            FeelingCategoryCreate { name: format!("CI_Cat_{}", uuid::Uuid::new_v4()) },
        )
            .await
            .unwrap()
    }

    #[tokio::test]
    async fn create_and_fetch_feeling() {
        let pool = get_test_pool().await;
        let cat_id = setup_category(&pool).await;

        let feeling_id = FeelingRepository::create(
            &pool,
            FeelingCreate { feeling_category_id: cat_id, name: "CI_Joie".to_string() },
        )
            .await
            .unwrap();

        assert!(feeling_id > 0);

        let row = FeelingRepository::by_id(&pool, feeling_id).await.unwrap();
        assert_eq!(row.name, "CI_Joie");
        assert_eq!(row.feeling_category_id, cat_id);

        // Cleanup (CASCADE supprime le feeling)
        let _ = FeelingCategoryRepository::delete(&pool, cat_id).await;
    }

    #[tokio::test]
    async fn update_feeling() {
        let pool = get_test_pool().await;
        let cat_id = setup_category(&pool).await;

        let feeling_id = FeelingRepository::create(
            &pool,
            FeelingCreate { feeling_category_id: cat_id, name: "CI_Original".to_string() },
        )
            .await
            .unwrap();

        FeelingRepository::update(
            &pool,
            feeling_id,
            FeelingUpdate { feeling_category_id: cat_id, name: "CI_Modifie".to_string() },
        )
            .await
            .unwrap();

        let row = FeelingRepository::by_id(&pool, feeling_id).await.unwrap();
        assert_eq!(row.name, "CI_Modifie");

        let _ = FeelingCategoryRepository::delete(&pool, cat_id).await;
    }

    #[tokio::test]
    async fn search_feeling_by_category() {
        let pool = get_test_pool().await;
        let cat_id = setup_category(&pool).await;

        let feeling_id = FeelingRepository::create(
            &pool,
            FeelingCreate { feeling_category_id: cat_id, name: "CI_Search".to_string() },
        )
            .await
            .unwrap();

        let params = FeelingSearchParams {
            feeling_category_id: Some(cat_id),
            name: None, start_at: None, end_at: None,
            cursor: None, page_size: Some(10),
        };

        let results = FeelingRepository::search(&pool, params, 10).await.unwrap();
        assert!(results.iter().any(|r| r.id == feeling_id));

        let _ = FeelingCategoryRepository::delete(&pool, cat_id).await;
    }

    #[tokio::test]
    async fn delete_feeling() {
        let pool = get_test_pool().await;
        let cat_id = setup_category(&pool).await;

        let feeling_id = FeelingRepository::create(
            &pool,
            FeelingCreate { feeling_category_id: cat_id, name: "CI_Del".to_string() },
        )
            .await
            .unwrap();

        FeelingRepository::delete(&pool, feeling_id).await.unwrap();
        let result = FeelingRepository::by_id(&pool, feeling_id).await;
        assert!(result.is_err());

        let _ = FeelingCategoryRepository::delete(&pool, cat_id).await;
    }
}