use chrono::Utc;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;
use crate::modules::article::model::{ArticleVisibility, ArticleWithAuthorRow};
use crate::modules::tag::dto::TagGet;
use crate::modules::tag::model::TagRow;
use crate::modules::user::dto::UserGetSimple;

#[derive(Serialize, ToSchema)]
pub struct ArticleGet {
    id: i32,
    author: UserGetSimple,
    title: String,
    content: String,
    is_deleted: bool,
    visibility: String,
    tags: Vec<String>,
    created_at: chrono::DateTime<Utc>,
    updated_at: chrono::DateTime<Utc>,
}

impl From<ArticleWithAuthorRow> for ArticleGet {
    fn from(article: ArticleWithAuthorRow) -> Self {
        Self {
            id: article.id,
            author: UserGetSimple {
                id: article.user_id,
                username: article.username,
                avatar: article.avatar,
                role: article.user_role.to_string()
            },
            title: article.title,
            content: article.content,
            is_deleted: article.is_deleted,
            visibility: article.visibility.to_string(),
            created_at: article.created_at,
            updated_at: article.updated_at,
            tags: article.tags,
        }
    }
}


#[derive(Deserialize, Validate, ToSchema)]
pub struct ArticleCreate {
    pub author_id: i32,
    #[validate(length(min = 10, max = 256))]
    pub title: String,
    #[validate(length(min = 300))]
    pub content: String,
    pub visibility: String,
    pub tags: Vec<i32>,
}

#[derive(Deserialize, Validate, ToSchema)]
pub struct ArticleUpdate {
    pub author_id: Option<i32>,
    #[validate(length(min = 10, max = 256))]
    pub title: Option<String>,
    #[validate(length(min = 300))]
    pub content: Option<String>,
    pub is_deleted: Option<bool>,
    pub visibility: Option<String>,
    pub tags: Option<Vec<i32>>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct ArticleSearchParams {
    pub author_id: Option<i32>,
    pub title: Option<String>,
    pub content: Option<String>,
    pub start_date: Option<chrono::DateTime<Utc>>,
    pub end_date: Option<chrono::DateTime<Utc>>,
    pub tag_ids: Option<Vec<i32>>,
    pub cursor: Option<chrono::DateTime<Utc>>,
    pub page_size: Option<i32>,
}




