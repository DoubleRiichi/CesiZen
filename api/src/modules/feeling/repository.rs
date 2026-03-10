use sqlx::{PgPool, QueryBuilder};
use crate::errors::app::AppError;
use crate::modules::feeling::dto::{FeelingCreate, FeelingSearchParams, FeelingUpdate};
use crate::modules::feeling::model::{FeelingWithCategoryRow};

pub struct FeelingRepository;

impl FeelingRepository {

    pub async fn by_id(pool: &PgPool, id: i32) -> Result<FeelingWithCategoryRow, sqlx::Error> {

        let feeling = sqlx::query_as::<_, FeelingWithCategoryRow>(
            r#"
            SELECT
                f.id,
                f.feeling_category_id,
                fc.name AS feeling_category_name,
                f.name
            FROM feeling f
            JOIN feeling_category fc
                ON fc.id = f.feeling_category_id
            WHERE f.id = $1
            "#
        )
            .bind(id)
            .fetch_one(pool)
            .await?;

        Ok(feeling)
    }

    pub async fn search(
        pool: &PgPool,
        params: FeelingSearchParams,
        page_size: i32,
    ) -> Result<Vec<FeelingWithCategoryRow>, AppError> {

        let mut qb = QueryBuilder::new(
            r#"
            SELECT
                f.id,
                f.feeling_category_id,
                fc.name AS feeling_category_name,
                f.name
            FROM feeling f
            JOIN feeling_category fc
                ON fc.id = f.feeling_category_id
            WHERE 1=1
            "#
        );

        if let Some(category_id) = params.feeling_category_id {
            qb.push(" AND f.feeling_category_id = ");
            qb.push_bind(category_id);
        }

        if let Some(name) = params.name {
            qb.push(" AND f.name ILIKE ");
            qb.push_bind(format!("%{}%", name));
        }

        qb.push(" ORDER BY f.id LIMIT ");
        qb.push_bind(page_size);

        let query = qb.build_query_as::<FeelingWithCategoryRow>();
        let rows = query.fetch_all(pool).await?;

        Ok(rows)
    }

    pub async fn create(
        pool: &PgPool,
        data: FeelingCreate,
    ) -> Result<i32, AppError> {

        let id: i32 = sqlx::query_scalar(
            r#"
            INSERT INTO feeling (feeling_category_id, name)
            VALUES ($1, $2)
            RETURNING id
            "#
        )
            .bind(data.feeling_category_id)
            .bind(data.name)
            .fetch_one(pool)
            .await?;

        Ok(id)
    }

    pub async fn update(
        pool: &PgPool,
        id: i32,
        data: FeelingUpdate,
    ) -> Result<(), AppError> {

        sqlx::query(
            r#"
            UPDATE feeling
            SET
                feeling_category_id = $1,
                name = $2
            WHERE id = $3
            "#
        )
            .bind(data.feeling_category_id)
            .bind(data.name)
            .bind(id)
            .execute(pool)
            .await?;

        Ok(())
    }

    pub async fn delete(pool: &PgPool, id: i32) -> Result<(), AppError> {

        sqlx::query("DELETE FROM feeling WHERE id = $1")
            .bind(id)
            .execute(pool)
            .await?;

        Ok(())
    }
}