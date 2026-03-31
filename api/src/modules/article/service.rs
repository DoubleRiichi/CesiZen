use crate::errors::app::AppError;
use crate::modules::article::dto::{ArticleCreate, ArticleGet, ArticleSearchParams, ArticleUpdate};
use crate::modules::article::repository::ArticleRepository;
use sqlx::PgPool;
use validator::Validate;

pub struct ArticleService;


impl ArticleService {

    pub async fn by_id(pool: &PgPool, article_id: i32) -> Result<ArticleGet, AppError> {

        let article = ArticleRepository::by_id(pool, article_id)
            .await?;

        Ok(article.into())
    }

    pub async fn search(pool: &PgPool, search_parameters: ArticleSearchParams) -> Result<Vec<ArticleGet>, AppError> {
        let page_size: i32;

        if let Some(size) = search_parameters.page_size {
            if size < 1 {
                return Err(AppError::Validation("Invalid page_size".to_string()))
            }
            if size > 500 {
                return Err(AppError::Validation("page_size count is too high".to_string()))
            }
            page_size = size;
        } else {
            page_size = 50;
        }
        let articles = ArticleRepository::search(pool, search_parameters, page_size)
            .await?;

        Ok(articles.into_iter().map(ArticleGet::from).collect())
    }


    //TODO: ensure only logged in admins and mods can create article, and use the user tied to the jwt token
    pub async fn create(pool: &PgPool, article: ArticleCreate) -> Result<i32, AppError> {
        article.validate()?;

        let created_article = ArticleRepository::create(pool, article)
          .await?;

        Ok(created_article.into())
    }

    pub async fn update(pool: &PgPool, article_id: i32, article: ArticleUpdate) -> Result<(), AppError> {
        article.validate()?;

        ArticleRepository::update(pool, article_id, article)
            .await?;

        Ok(())
    }

    pub async fn delete(pool: &PgPool, article_id: i32) -> Result<(), AppError> {

        let result = ArticleRepository::delete(pool, article_id)
            .await?;

        Ok(result)
    }
}