use crate::errors::app::AppError;
use crate::modules::tag::dto::TagCreate;
use crate::modules::tag::model::TagRow;
use sqlx::PgPool;

pub struct TagRepository;

impl TagRepository {
    pub async fn find_by_id(pool: &PgPool, id: i32) -> Result<TagRow, AppError> {
        let tag = sqlx::query_as::<_, TagRow>(
        r#"select * from tag where "id" = $1"#,
        )
            .bind(id)
            .fetch_one(pool)
            .await?;

        Ok(tag)
    }

    pub async fn index(pool: &PgPool) -> Result<Vec<TagRow>, AppError> {
        let tags = sqlx::query_as::<_, TagRow>(
        r#"select * from tag order by id desc"#)
            .fetch_all(pool)
            .await?;

        Ok(tags)
    }

    pub async fn create(pool: &PgPool, tag: TagCreate) -> Result<TagRow, sqlx::Error> {
        let tag = sqlx::query_as::<_, TagRow>(
            r#"INSERT INTO tag
            (name)
            VALUES ($1)
            RETURNING *"#)
            .bind(tag.name)
            .fetch_one(pool)
            .await?;
        
        Ok(tag)
    }

    pub async fn delete(pool: &PgPool, id: i32) -> Result<bool, AppError> {
        sqlx::query(
            r#"delete from tag where "id" = $1"#
        )
            .bind(id)
            .execute(pool)
            .await?;

        Ok(true)
    }

    pub async fn find_from_article(pool: &PgPool, article_id: i32) -> Result<Vec<TagRow>, AppError> {
        let tags = sqlx::query_as::<_, TagRow>(
            r#"SELECT t.id, t.name
                FROM "article_tag" at
                JOIN tag t ON t.id = at.tag_id
                WHERE at.article_id = $1"#,
            )
            .bind(article_id)
            .fetch_all(pool)
            .await?;

        Ok(tags)
    }
}