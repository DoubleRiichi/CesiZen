use crate::errors::app::AppError;
use crate::modules::feeling::dto::{FeelingCreate, FeelingGet, FeelingSearchParams, FeelingUpdate};
use crate::modules::feeling::repository::FeelingRepository;
use sqlx::PgPool;
use validator::Validate;

pub struct FeelingService;


impl FeelingService {

    pub async fn by_id(pool: &PgPool, feeling_id: i32) -> Result<FeelingGet, AppError> {

        let feeling = FeelingRepository::by_id(pool, feeling_id)
            .await?;

        Ok(feeling.into())
    }

    pub async fn search(pool: &PgPool, search_parameters: FeelingSearchParams) -> Result<Vec<FeelingGet>, AppError> {
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
        let feelings = FeelingRepository::search(pool, search_parameters, page_size)
            .await?;

        Ok(feelings.into_iter().map(FeelingGet::from).collect())
    }


    //TODO: ensure only logged in admins and mods can create feeling, and use the user tied to the jwt token
    pub async fn create(pool: &PgPool, feeling: FeelingCreate) -> Result<i32, AppError> {
        feeling.validate()?;

        let created_feeling = FeelingRepository::create(pool, feeling)
            .await?;

        Ok(created_feeling.into())
    }

    pub async fn update(pool: &PgPool, feeling_id: i32, feeling: FeelingUpdate) -> Result<(), AppError> {
        feeling.validate()?;

        FeelingRepository::update(pool, feeling_id, feeling)
            .await?;

        Ok(())
    }

    pub async fn delete(pool: &PgPool, feeling_id: i32) -> Result<(), AppError> {

        let result = FeelingRepository::delete(pool, feeling_id)
            .await?;

        Ok(result)
    }
}