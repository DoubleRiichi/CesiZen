// ============================================================
// Tests — Module FeelingTracker
// ============================================================

#[cfg(test)]
mod unit {
    use chrono::{DateTime, Utc};
    use crate::modules::feeling_tracker::dto::{
        FeelingTrackerCreate, FeelingTrackerGet, FeelingTrackerSearchParams, FeelingTrackerUpdate,
    };
    use crate::modules::feeling_tracker::model::FeelingTrackerWithFeelingRow;

    fn fake_dt() -> DateTime<Utc> {
        chrono::DateTime::from_timestamp(1_700_000_000, 0).unwrap()
    }

    fn fake_row() -> FeelingTrackerWithFeelingRow {
        FeelingTrackerWithFeelingRow {
            id: 10,
            user_id: 1,
            feeling_id: 2,
            feeling_name: "Heureux".to_string(),
            feeling_category_id: 1,
            feeling_category_name: "Positif".to_string(),
            timestamp_start: fake_dt(),
            timestamp_end: fake_dt(),
            intensity: 8,
            notes: "Bonne journée".to_string(),
            location: "Paris".to_string(),
            created_at: fake_dt(),
            updated_at: fake_dt(),
        }
    }

    #[test]
    fn feeling_tracker_get_from_row() {
        let dto = FeelingTrackerGet::from(fake_row());
        assert_eq!(dto.id, 10);
        assert_eq!(dto.user_id, 1);
        assert_eq!(dto.feeling, "Heureux");
        assert_eq!(dto.feeling_category, "Positif");
        assert_eq!(dto.intensity, 8);
        assert_eq!(dto.notes, "Bonne journée");
        assert_eq!(dto.location, "Paris");
    }

    #[test]
    fn feeling_tracker_create_deserializes() {
        let json = serde_json::json!({
            "user_id": 1,
            "feeling_id": 2,
            "feeling_category_id": 1,
            "timestamp_start": "2024-01-01T10:00:00Z",
            "timestamp_end": "2024-01-01T11:00:00Z",
            "intensity": 7,
            "notes": "test notes",
            "location": "Lyon"
        });
        let dto: FeelingTrackerCreate = serde_json::from_value(json).unwrap();
        assert_eq!(dto.user_id, 1);
        assert_eq!(dto.feeling_id, 2);
        assert_eq!(dto.intensity, 7);
    }

    #[test]
    fn feeling_tracker_update_deserializes() {
        let json = serde_json::json!({
            "user_id": 1,
            "feeling_id": 3,
            "feeling_category_id": 2,
            "timestamp_start": "2024-06-01T08:00:00Z",
            "timestamp_end": "2024-06-01T09:00:00Z",
            "intensity": 5,
            "notes": "",
            "location": ""
        });
        let dto: FeelingTrackerUpdate = serde_json::from_value(json).unwrap();
        assert_eq!(dto.feeling_id, 3);
        assert_eq!(dto.intensity, 5);
    }

    #[test]
    fn feeling_tracker_search_params_all_optional() {
        let json = r#"{}"#;
        let dto: FeelingTrackerSearchParams = serde_json::from_str(json).unwrap();
        assert!(dto.user_id.is_none());
        assert!(dto.feeling_id.is_none());
        assert!(dto.page_size.is_none());
    }

    #[test]
    fn feeling_tracker_search_params_with_user_id() {
        let json = r#"{"user_id":42,"page_size":25}"#;
        let dto: FeelingTrackerSearchParams = serde_json::from_str(json).unwrap();
        assert_eq!(dto.user_id, Some(42));
        assert_eq!(dto.page_size, Some(25));
    }

    // ── Règles métier : intensité 1-10 ────────────────────────

    #[test]
    fn intensity_boundary_min_valid() {
        // i16 = 1 est valide selon la contrainte DB (CHECK intensity BETWEEN 1 AND 10)
        // Le DTO accepte i16 — la validation se fait au niveau SQL
        // On vérifie au moins que i16 = 1 se désérialise correctement
        let json = serde_json::json!({
            "user_id": 1, "feeling_id": 1, "feeling_category_id": 1,
            "timestamp_start": "2024-01-01T00:00:00Z",
            "timestamp_end": "2024-01-01T01:00:00Z",
            "intensity": 1, "notes": "", "location": ""
        });
        let dto: FeelingTrackerCreate = serde_json::from_value(json).unwrap();
        assert_eq!(dto.intensity, 1i16);
    }

    #[test]
    fn intensity_boundary_max_valid() {
        let json = serde_json::json!({
            "user_id": 1, "feeling_id": 1, "feeling_category_id": 1,
            "timestamp_start": "2024-01-01T00:00:00Z",
            "timestamp_end": "2024-01-01T01:00:00Z",
            "intensity": 10, "notes": "", "location": ""
        });
        let dto: FeelingTrackerCreate = serde_json::from_value(json).unwrap();
        assert_eq!(dto.intensity, 10i16);
    }
}

// ── Tests d'intégration ───────────────────────────────────────

#[cfg(test)]
#[cfg(feature = "integration")]
mod integration {
    use chrono::{Utc, DateTime};
    use sqlx::PgPool;
    use crate::modules::feeling_category::dto::FeelingCategoryCreate;
    use crate::modules::feeling_category::repository::FeelingCategoryRepository;
    use crate::modules::feeling::dto::FeelingCreate;
    use crate::modules::feeling::repository::FeelingRepository;
    use crate::modules::feeling_tracker::dto::{
        FeelingTrackerCreate, FeelingTrackerSearchParams, FeelingTrackerUpdate,
    };
    use crate::modules::feeling_tracker::repository::FeelingTrackerRepository;

    async fn get_test_pool() -> PgPool {
        let url = std::env::var("DATABASE_TEST_URL").unwrap();
        PgPool::connect(&url).await.unwrap()
    }

    fn fake_dt_start() -> DateTime<Utc> {
        chrono::DateTime::from_timestamp(1_700_000_000, 0).unwrap()
    }

    fn fake_dt_end() -> DateTime<Utc> {
        chrono::DateTime::from_timestamp(1_700_003_600, 0).unwrap()
    }

    /// Crée une catégorie + un feeling et retourne (cat_id, feeling_id)
    async fn setup_feeling(pool: &PgPool) -> (i32, i32) {
        let cat_id = FeelingCategoryRepository::create(
            pool,
            FeelingCategoryCreate {
                name: format!("CI_TrackerCat_{}", uuid::Uuid::new_v4()),
            },
        )
            .await
            .unwrap();

        let feeling_id = FeelingRepository::create(
            pool,
            FeelingCreate {
                feeling_category_id: cat_id,
                name: format!("CI_TrackerFeeling_{}", uuid::Uuid::new_v4()),
            },
        )
            .await
            .unwrap();

        (cat_id, feeling_id)
    }

    fn tracker_create(user_id: i32, feeling_id: i32, cat_id: i32) -> FeelingTrackerCreate {
        FeelingTrackerCreate {
            user_id,
            feeling_id,
            timestamp_start: fake_dt_start(),
            timestamp_end: fake_dt_end(),
            intensity: 7,
            notes: "CI note".to_string(),
            location: "Paris".to_string(),
        }
    }

    #[tokio::test]
    async fn create_and_fetch_feeling_tracker() {
        let pool = get_test_pool().await;
        let (cat_id, feeling_id) = setup_feeling(&pool).await;

        let tracker_id = FeelingTrackerRepository::create(
            &pool,
            tracker_create(1, feeling_id, cat_id),
        )
            .await
            .unwrap();

        assert!(tracker_id > 0);

        let row = FeelingTrackerRepository::by_id(&pool, tracker_id).await.unwrap();
        assert_eq!(row.user_id, 1);
        assert_eq!(row.feeling_id, feeling_id);
        assert_eq!(row.intensity, 7);
        assert_eq!(row.location, "Paris");

        // Cleanup
        let _ = FeelingTrackerRepository::delete(&pool, tracker_id).await;
        let _ = FeelingCategoryRepository::delete(&pool, cat_id).await;
    }

    #[tokio::test]
    async fn update_feeling_tracker() {
        let pool = get_test_pool().await;
        let (cat_id, feeling_id) = setup_feeling(&pool).await;

        let tracker_id = FeelingTrackerRepository::create(
            &pool,
            tracker_create(1, feeling_id, cat_id),
        )
            .await
            .unwrap();
        let user_id = 1;
        let update = FeelingTrackerUpdate {
            user_id,
            feeling_id,
            timestamp_start: fake_dt_start(),
            timestamp_end: fake_dt_end(),
            intensity: 3,
            notes: "Updated".to_string(),
            location: "Lyon".to_string(),
        };

        FeelingTrackerRepository::update(&pool, tracker_id, update).await.unwrap();

        let row = FeelingTrackerRepository::by_id(&pool, tracker_id).await.unwrap();
        assert_eq!(row.intensity, 3);
        assert_eq!(row.location, "Lyon");
        assert_eq!(row.notes, "Updated");

        let _ = FeelingTrackerRepository::delete(&pool, tracker_id).await;
        let _ = FeelingCategoryRepository::delete(&pool, cat_id).await;
    }

    #[tokio::test]
    async fn delete_feeling_tracker() {
        let pool = get_test_pool().await;
        let (cat_id, feeling_id) = setup_feeling(&pool).await;

        let tracker_id = FeelingTrackerRepository::create(
            &pool,
            tracker_create(1, feeling_id, cat_id),
        )
            .await
            .unwrap();

        FeelingTrackerRepository::delete(&pool, tracker_id).await.unwrap();
        let result = FeelingTrackerRepository::by_id(&pool, tracker_id).await;
        assert!(result.is_err());

        let _ = FeelingCategoryRepository::delete(&pool, cat_id).await;
    }

    #[tokio::test]
    async fn search_trackers_by_user() {
        let pool = get_test_pool().await;
        let (cat_id, feeling_id) = setup_feeling(&pool).await;

        let tracker_id = FeelingTrackerRepository::create(
            &pool,
            tracker_create(1, feeling_id, cat_id),
        )
            .await
            .unwrap();

        let params = FeelingTrackerSearchParams {
            user_id: Some(1),
            feeling_id: None, start_date: None,
            end_date: None, cursor: None, page_size: Some(50),
        };

        let results = FeelingTrackerRepository::search(&pool, params, 50).await.unwrap();
        assert!(results.iter().any(|r| r.id == tracker_id));

        let _ = FeelingTrackerRepository::delete(&pool, tracker_id).await;
        let _ = FeelingCategoryRepository::delete(&pool, cat_id).await;
    }

    #[tokio::test]
    async fn search_trackers_by_feeling_id() {
        let pool = get_test_pool().await;
        let (cat_id, feeling_id) = setup_feeling(&pool).await;

        let tracker_id = FeelingTrackerRepository::create(
            &pool,
            tracker_create(1, feeling_id, cat_id),
        )
            .await
            .unwrap();

        let params = FeelingTrackerSearchParams {
            user_id: None,
            feeling_id: Some(feeling_id),
            start_date: None, end_date: None,
            cursor: None, page_size: Some(10),
        };

        let results = FeelingTrackerRepository::search(&pool, params, 10).await.unwrap();
        assert!(!results.is_empty());
        for r in &results {
            assert_eq!(r.feeling_id, feeling_id);
        }

        let _ = FeelingTrackerRepository::delete(&pool, tracker_id).await;
        let _ = FeelingCategoryRepository::delete(&pool, cat_id).await;
    }
}