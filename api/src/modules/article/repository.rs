use crate::errors::app::AppError;
use crate::modules::article::dto::{ArticleCreate, ArticleSearchParams, ArticleUpdate};
use crate::modules::article::model::ArticleWithAuthorRow;
use sqlx::PgPool;

pub struct ArticleRepository;

impl ArticleRepository {
    pub async fn by_id(pool: &PgPool, id: i32) -> Result<ArticleWithAuthorRow, sqlx::Error> {
        let article = sqlx::query_as::<_, ArticleWithAuthorRow>(
            r#"
            SELECT
                a.id,
                a.title,
                a.content,
                a.is_deleted,
                a.created_at,
                a.updated_at,
                u.id AS user_id,
                u.username,
                u.role AS user_role,
                u.avatar,
                a.visibility,
                COALESCE(ARRAY_AGG(t.name) FILTER (WHERE t.id IS NOT NULL), '{}') AS tags
            FROM article a
            JOIN "user" u ON a.author_id = u.id
            LEFT JOIN article_tag at ON at.article_id = a.id
            LEFT JOIN tag t ON t.id = at.tag_id
            WHERE a.id = $1
            GROUP BY a.id, u.id
            "#,
        )
            .bind(id)
            .fetch_one(pool)
            .await?;
        println!("{:?}", article);
        Ok(article)
    }

    pub async fn search(
        pool: &PgPool,
        params: ArticleSearchParams,
        page_size: i32,
    ) -> Result<Vec<ArticleWithAuthorRow>, AppError> {
        let mut qb = sqlx::QueryBuilder::new(
            r#"
            SELECT
                a.id,
                a.title,
                a.content,
                a.is_deleted,
                a.created_at,
                a.updated_at,
                u.id AS user_id,
                u.username,
                u.role AS user_role,
                u.avatar,
                a.visibility,
                COALESCE(ARRAY_AGG(t.name) FILTER (WHERE t.id IS NOT NULL), '{}') AS "tags"
            FROM article a
            JOIN "user" u ON a.author_id = u.id
            LEFT JOIN article_tag at ON at.article_id = a.id
            LEFT JOIN tag t ON t.id = at.tag_id
            WHERE 1=1
        "#,
        );

        // Dynamic filters
        if let Some(author_id) = params.author_id {
            qb.push(" AND a.author_id = ");
            qb.push_bind(author_id);
        }

        if let Some(title) = params.title {
            qb.push(" AND a.title ILIKE ");
            qb.push_bind(format!("%{}%", title));
        }

        if let Some(content) = params.content {
            qb.push(" AND a.content ILIKE ");
            qb.push_bind(format!("%{}%", content));
        }

        if let Some(start) = params.start_date {
            qb.push(" AND a.created_at >= ");
            qb.push_bind(start);
        }

        if let Some(end) = params.end_date {
            qb.push(" AND a.created_at <= ");
            qb.push_bind(end);
        }

        if let Some(tag_ids) = params.tag_ids {
            if !tag_ids.is_empty() {
                qb.push(" AND at.tag_id = ANY(");
                qb.push_bind(tag_ids);
                qb.push(")");
            }
        }

        if let Some(cursor) = params.cursor {
            qb.push(" AND a.created_at < ");
            qb.push_bind(cursor);
        }

        // Final clauses (GROUP BY must come AFTER WHERE)
        qb.push(" GROUP BY a.id, u.id ORDER BY a.created_at DESC LIMIT ");
        qb.push_bind(page_size);

        let query = qb.build_query_as::<ArticleWithAuthorRow>();
        let articles = query.fetch_all(pool).await?;

        Ok(articles)
    }

    pub async fn create(
        pool: &PgPool,
        data: ArticleCreate,
    ) -> Result<i32, AppError> {
        let mut tx = pool.begin().await?;

        let article_id: i32 = sqlx::query_scalar(
            r#"
            INSERT INTO article (title, content, author_id, visibility, is_deleted, created_at, updated_at)
            VALUES ($1, $2, $3, $4::article_visibility, false, NOW(), NOW())
            RETURNING id
            "#,
        )
            .bind(data.title)
            .bind(data.content)
            .bind(data.author_id)
            .bind(data.visibility)
            .fetch_one(&mut *tx)
            .await?;

        for tag_id in data.tags {
            sqlx::query(
                r#"INSERT INTO article_tag (article_id, tag_id) VALUES ($1, $2)"#,
            )
                .bind(article_id)
                .bind(tag_id)
                .execute(&mut *tx)
                .await?;
        }

        tx.commit().await?;
        Ok(article_id)
    }

    pub async fn update(
        pool: &PgPool,
        article_id: i32,
        data: ArticleUpdate,
    ) -> Result<(), AppError> {
        let mut tx = pool.begin().await?;

        if data.title.is_some() || data.content.is_some() || data.visibility.is_some() {
            sqlx::query(
                r#"
                UPDATE article
                SET
                    title       = COALESCE($1, title),
                    content     = COALESCE($2, content),
                    visibility  = COALESCE($3::article_visibility, visibility),
                    updated_at  = NOW()
                WHERE id = $4
                "#,
            )
                .bind(data.title)
                .bind(data.content)
                .bind(data.visibility)
                .bind(article_id)
                .execute(&mut *tx)
                .await?;
        }

        if let Some(tag_ids) = data.tags {
            sqlx::query(
                r#"DELETE FROM article_tag WHERE article_id = $1"#,
            )
                .bind(article_id)
                .execute(&mut *tx)
                .await?;

            for tag_id in tag_ids {
                sqlx::query(
                    r#"INSERT INTO article_tag (article_id, tag_id) VALUES ($1, $2)"#,
                )
                    .bind(article_id)
                    .bind(tag_id)
                    .execute(&mut *tx)
                    .await?;
            }
        }

        tx.commit().await?;
        Ok(())
    }

    pub async fn delete(pool: &PgPool, id: i32) -> Result<(), AppError> {
        sqlx::query(r#"DELETE FROM article WHERE id = $1"#)
            .bind(id)
            .execute(pool)
            .await?;

        Ok(())
    }
}