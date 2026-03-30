use sqlx::{PgPool, QueryBuilder};
use crate::errors::app::AppError;
use crate::modules::feeling_tracker::dto::{FeelingTrackerCreate, FeelingTrackerSearchParams, FeelingTrackerUpdate};
use crate::modules::feeling_tracker::model::FeelingTrackerWithFeelingRow;

pub struct FeelingTrackerRepository;

impl FeelingTrackerRepository {

    pub async fn by_id(pool: &PgPool, id: i32) -> Result<FeelingTrackerWithFeelingRow, sqlx::Error> {

        let row = sqlx::query_as::<_, FeelingTrackerWithFeelingRow>(
            r#"
            SELECT
                ft.id,
                ft.user_id,
                ft.feeling_id,
                f.name AS feeling_name,
                fc.id AS feeling_category_id,
                fc.name AS feeling_category_name,
                ft.timestamp_start,
                ft.timestamp_end,
                ft.intensity,
                ft.notes,
                ft.location,
                ft.created_at,
                ft.updated_at
            FROM feeling_tracker ft
            JOIN feeling f ON f.id = ft.feeling_id
            JOIN feeling_category fc ON fc.id = f.feeling_category_id
            WHERE ft.id = $1
            "#
        )
            .bind(id)
            .fetch_one(pool)
            .await?;

        Ok(row)
    }

    pub async fn search(
        pool: &PgPool,
        params: FeelingTrackerSearchParams,
        page_size: i32,
    ) -> Result<Vec<FeelingTrackerWithFeelingRow>, AppError> {

        let mut qb = QueryBuilder::new(
            r#"
            SELECT
                ft.id,
                ft.user_id,
                ft.feeling_id,
                f.name AS feeling_name,
                fc.id AS feeling_category_id,
                fc.name AS feeling_category_name,
                ft.timestamp_start,
                ft.timestamp_end,
                ft.intensity,
                ft.notes,
                ft.location,
                ft.created_at,
                ft.updated_at
            FROM feeling_tracker ft
            JOIN feeling f ON f.id = ft.feeling_id
            JOIN feeling_category fc ON fc.id = f.feeling_category_id
            WHERE 1=1
            "#
        );

        if let Some(user_id) = params.user_id {
            qb.push(" AND ft.user_id = ");
            qb.push_bind(user_id);
        }

        if let Some(feeling_id) = params.feeling_id {
            qb.push(" AND ft.feeling_id = ");
            qb.push_bind(feeling_id);
        }

        if let Some(start) = params.start_date {
            qb.push(" AND ft.timestamp_start >= ");
            qb.push_bind(start);
        }

        if let Some(end) = params.end_date {
            qb.push(" AND ft.timestamp_end <= ");
            qb.push_bind(end);
        }

        if let Some(cursor) = params.cursor {
            qb.push(" AND ft.created_at < ");
            qb.push_bind(cursor);
        }

        qb.push(" ORDER BY ft.created_at DESC LIMIT ");
        qb.push_bind(page_size);

        let query = qb.build_query_as::<FeelingTrackerWithFeelingRow>();
        let rows = query.fetch_all(pool).await?;

        Ok(rows)
    }

    pub async fn create(
        pool: &PgPool,
        data: FeelingTrackerCreate,
    ) -> Result<i32, AppError> {

        let id: i32 = sqlx::query_scalar(
            r#"
            INSERT INTO feeling_tracker
            (user_id, feeling_id, timestamp_start, timestamp_end, intensity, notes, location)
            VALUES ($1,$2,$3,$4,$5,$6,$7)
            RETURNING id
            "#
        )
            .bind(data.user_id)
            .bind(data.feeling_id)
            .bind(data.timestamp_start)
            .bind(data.timestamp_end)
            .bind(data.intensity)
            .bind(data.notes)
            .bind(data.location)
            .fetch_one(pool)
            .await?;

        Ok(id)
    }

    pub async fn update(
        pool: &PgPool,
        id: i32,
        data: FeelingTrackerUpdate,
    ) -> Result<(), AppError> {

        sqlx::query(
            r#"
            UPDATE feeling_tracker
            SET
                user_id = $1,
                feeling_id = $2,
                timestamp_start = $3,
                timestamp_end = $4,
                intensity = $5,
                notes = $6,
                location = $7
            WHERE id = $8
            "#
        )
            .bind(data.user_id)
            .bind(data.feeling_id)
            .bind(data.timestamp_start)
            .bind(data.timestamp_end)
            .bind(data.intensity)
            .bind(data.notes)
            .bind(data.location)
            .bind(id)
            .execute(pool)
            .await?;

        Ok(())
    }

    pub async fn delete(pool: &PgPool, id: i32) -> Result<(), AppError> {

        sqlx::query("DELETE FROM feeling_tracker WHERE id = $1")
            .bind(id)
            .execute(pool)
            .await?;

        Ok(())
    }
}