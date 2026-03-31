use crate::errors::app::AppError;
use crate::modules::user::dto::{UserCreate, UserGet, UserSearchParams, UserUpdate};
use crate::modules::user::model::UserRole;
use crate::modules::user::repository::UserRepository;
use sqlx::PgPool;
use validator::Validate;

pub struct UserService;


fn hash(plain_password: &str) -> Result<String, AppError> {
    let hashed = bcrypt::hash(plain_password, bcrypt::DEFAULT_COST)
        .map_err(|_| AppError::Internal("Error during user creation".to_string()))?;

    Ok(hashed)
}

impl UserService {

    pub async fn by_id(pool: &PgPool, id: i32) -> Result<UserGet, AppError> {

        let user_row = UserRepository::by_id(pool, id)
            .await?;

        Ok(user_row.into())
    }

    pub async fn delete(pool: &PgPool, id: i32) -> Result<(), AppError> {
        let result = UserRepository::delete(pool, id)
            .await?;

        Ok(result)
    }

    pub async fn create(pool: &PgPool, user: UserCreate) -> Result<UserGet, AppError> {
        user.validate()?;

        let hashed = self::hash(&*user.password)?;

        let user_row = UserRepository::create(pool, user, &UserRole::User, &hashed)
            .await
            .map_err(|_| AppError::Internal("Error creating user".to_string()))?;

        Ok(user_row.into())
    }

    pub async fn update(pool: &PgPool, id: i32, user: UserUpdate) -> Result<UserGet, AppError> {
        user.validate()?;

        let hashed = self::hash(&*user.password)?;

        let user_row = UserRepository::update(pool, id, user, &hashed)
            .await
            .map_err(|_| AppError::Internal("Error updating user".to_string()))?;

        Ok(user_row.into())
    }


    pub async fn change_role(pool: &PgPool, id: i32, role: UserRole) -> Result<UserGet, AppError> {

        let user_row = UserRepository::change_role(pool, id, &role)
            .await
            .map_err(|_| AppError::Internal("Error updating user role".to_string()))?;

        Ok(user_row.into())
    }

    pub async fn find_by_email(pool: &PgPool, email: &str) -> Result<UserGet, AppError> {

        let user_row = UserRepository::by_mail(pool, email)
            .await?;

        Ok(user_row.into())
    }

    pub async fn find_all(pool: &PgPool) -> Result<Vec<UserGet>, AppError> {

        let user_rows = UserRepository::index(pool)
            .await?;

        Ok(user_rows.into_iter().map(UserGet::from).collect())
    }

    pub async fn search(pool: &PgPool, search_parameters: UserSearchParams) -> Result<Vec<UserGet>, AppError> {
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

        let result = UserRepository::search(pool, search_parameters, page_size)
            .await?;

        Ok(result.into_iter().map(UserGet::from).collect())
    }
}