use sqlx::{PgPool, QueryBuilder};
use crate::errors::app::AppError;

use crate::modules::feeling_category::dto::{FeelingCategoryCreate, FeelingCategorySearchParams, FeelingCategoryUpdate};
use crate::modules::feeling_category::model::FeelingCategoryRow;

pub struct FeelingCategoryRepository;

impl FeelingCategoryRepository {

    pub async fn by_id(pool: &PgPool, id: i32) -> Result<FeelingCategoryRow, sqlx::Error> {
        let category = sqlx::query_as::<_, FeelingCategoryRow>(
            r#"
            SELECT id, name
            FROM feeling_category
            WHERE id = $1
            "#
        )
            .bind(id)
            .fetch_one(pool)
            .await?;

        Ok(category)
    }

    pub async fn search(
        pool: &PgPool,
        params: FeelingCategorySearchParams,
        page_size: i32,
    ) -> Result<Vec<FeelingCategoryRow>, AppError> {

        let mut qb = QueryBuilder::new(
            r#"
            SELECT id, name
            FROM feeling_category
            WHERE 1=1
            "#
        );

        if let Some(name) = params.name {
            qb.push(" AND name ILIKE ");
            qb.push_bind(format!("%{}%", name));
        }

        if let Some(start) = params.start_at {
            qb.push(" AND created_at >= ");
            qb.push_bind(start);
        }

        if let Some(end) = params.end_at {
            qb.push(" AND a.created_at <= ");
            qb.push_bind(end);
        }


        if let Some(cursor) = params.cursor {
            qb.push(" AND a.created_at < ");
            qb.push_bind(cursor);
        }

        qb.push(" ORDER BY id LIMIT ");
        qb.push_bind(page_size);

        let query = qb.build_query_as::<FeelingCategoryRow>();
        let rows = query.fetch_all(pool).await?;

        Ok(rows)
    }

    pub async fn create(
        pool: &PgPool,
        data: FeelingCategoryCreate,
    ) -> Result<i32, AppError> {

        let id: i32 = sqlx::query_scalar(
            r#"
            INSERT INTO feeling_category (name)
            VALUES ($1)
            RETURNING id
            "#
        )
            .bind(data.name)
            .fetch_one(pool)
            .await?;

        Ok(id)
    }

    pub async fn update(
        pool: &PgPool,
        id: i32,
        data: FeelingCategoryUpdate,
    ) -> Result<(), AppError> {

        sqlx::query(
            r#"
            UPDATE feeling_category
            SET name = $1
            WHERE id = $2
            "#
        )
            .bind(data.name)
            .bind(id)
            .execute(pool)
            .await?;

        Ok(())
    }

    pub async fn delete(pool: &PgPool, id: i32) -> Result<(), AppError> {

        sqlx::query("DELETE FROM feeling_category WHERE id = $1")
            .bind(id)
            .execute(pool)
            .await?;

        Ok(())
    }
}