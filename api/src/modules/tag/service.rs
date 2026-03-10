use axum::extract::Path;
use sqlx::PgPool;
use utoipa::openapi::Paths;
use crate::errors::app::AppError;
use crate::modules::tag::dto::{TagCreate, TagGet};
use crate::modules::tag::repository::TagRepository;

pub struct TagService;


impl TagService {
    pub async fn find_all(pool: &PgPool) -> Result<Vec<TagGet>, AppError> {

        let tags = TagRepository::index(pool)
            .await?;

        Ok(tags.into_iter().map(TagGet::from).collect())
    }

    pub async fn by_id(pool: &PgPool, id: i32) -> Result<TagGet, AppError> {
        let tag = TagRepository::find_by_id(pool, id)
            .await?;

        Ok(tag.into())
    }

    pub async fn create(pool: &PgPool, input: TagCreate) -> Result<TagGet, AppError> {
        let tag = TagRepository::create(pool, input)
            .await?;

        Ok(tag.into())
    }

    pub async fn delete(pool: &PgPool, id: i32) -> Result<(), AppError> {
        TagRepository::delete(pool, id)
            .await?;

        Ok(())
    }


}
