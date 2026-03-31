use crate::errors::app::AppError;
use crate::modules::feeling_tracker::dto::{FeelingTrackerCreate, FeelingTrackerGet, FeelingTrackerSearchParams, FeelingTrackerUpdate};
use crate::modules::feeling_tracker::repository::FeelingTrackerRepository;
use sqlx::PgPool;
use validator::Validate;

pub struct FeelingTrackerService {}


impl FeelingTrackerService {

    pub async fn by_id(pool: &PgPool,
                       feeling_tracker_id: i32) -> Result<FeelingTrackerGet, AppError> {
        let feeling_tracker = FeelingTrackerRepository::by_id(pool, feeling_tracker_id)
            .await?;

        Ok(feeling_tracker.into())
    }

    pub async fn search(pool: &PgPool, search_parameters: FeelingTrackerSearchParams) -> Result<Vec<FeelingTrackerGet>, AppError> {
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
        let feeling_trackers_entries = FeelingTrackerRepository::search(pool, search_parameters, page_size)
            .await?;

        Ok(feeling_trackers_entries.into_iter().map(FeelingTrackerGet::from).collect())
    }


    //TODO: ensure only logged in admins and mods can create feeling, and use the user tied to the jwt token
    pub async fn create(pool: &PgPool, feeling_tracker: FeelingTrackerCreate) -> Result<i32, AppError> {
        feeling_tracker.validate()?;

        let created_feeling_tracker = FeelingTrackerRepository::create(pool, feeling_tracker)
            .await?;

        Ok(created_feeling_tracker.into())
    }

    pub async fn update(pool: &PgPool, feeling_tracker_id: i32, feeling_tracker: FeelingTrackerUpdate) -> Result<(), AppError> {
        feeling_tracker.validate()?;

        FeelingTrackerRepository::update(pool, feeling_tracker_id, feeling_tracker)
            .await?;

        Ok(())
    }

    pub async fn delete(pool: &PgPool, feeling_tracker_id: i32) -> Result<(), AppError> {

        let result = FeelingTrackerRepository::delete(pool, feeling_tracker_id)
            .await?;

        Ok(result)
    }
}