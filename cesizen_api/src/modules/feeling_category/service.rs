use crate::errors::app::AppError;
use crate::modules::feeling_category::dto::{FeelingCategoryCreate, FeelingCategoryGet, FeelingCategorySearchParams, FeelingCategoryUpdate};
use crate::modules::feeling_category::repository::FeelingCategoryRepository;
use sqlx::PgPool;
use validator::Validate;

pub struct FeelingCategoryService;

impl FeelingCategoryService {

    pub async fn by_id(pool: &PgPool, feeling_category_id: i32) -> Result<FeelingCategoryGet, AppError> {

        let feeling_category = FeelingCategoryRepository::by_id(pool, feeling_category_id)
            .await?;

        Ok(feeling_category.into())
    }

    pub async fn search(pool: &PgPool, search_parameters: FeelingCategorySearchParams) -> Result<Vec<FeelingCategoryGet>, AppError> {
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
        let feeling_categories = FeelingCategoryRepository::search(pool, search_parameters, page_size)
            .await?;

        Ok(feeling_categories.into_iter().map(FeelingCategoryGet::from).collect())
    }


    //TODO: ensure only logged in admins and mods can create feeling, and use the user tied to the jwt token
    pub async fn create(pool: &PgPool, feeling_category: FeelingCategoryCreate) -> Result<i32, AppError> {
        feeling_category.validate()?;

        let feeling_category_created = FeelingCategoryRepository::create(pool, feeling_category)
            .await?;

        Ok(feeling_category_created)
    }

    pub async fn update(pool: &PgPool, feeling_category_id: i32, feeling_category: FeelingCategoryUpdate) -> Result<(), AppError> {
        feeling_category.validate()?;

        FeelingCategoryRepository::update(pool, feeling_category_id, feeling_category)
            .await?;

        Ok(())
    }

    pub async fn delete(pool: &PgPool, feeling_category_id: i32) -> Result<(), AppError> {

        FeelingCategoryRepository::delete(pool, feeling_category_id)
            .await?;

        Ok(())
    }
}