use std::fmt;
use chrono::Utc;
use serde::Deserialize;
use crate::modules::tag::model::TagRow;
use crate::modules::user::model::UserRole;

#[derive(sqlx::FromRow)]
pub struct ArticleRow {
    pub id: i32,
    pub user_id: i32,
    pub title: String,
    pub content: String,
    pub is_deleted: bool,
    pub visibility: ArticleVisibility,
    pub created_at: chrono::DateTime<Utc>,
    pub updated_at: chrono::DateTime<Utc>,
}

#[derive(sqlx::FromRow, Debug)]
pub struct ArticleWithAuthorRow {
    pub id: i32,
    pub title: String,
    pub content: String,
    pub is_deleted: bool,
    pub created_at: chrono::DateTime<Utc>,
    pub updated_at: chrono::DateTime<Utc>,
    pub user_id: i32,
    pub username: String,
    pub user_role: UserRole,
    pub avatar: String,
    pub visibility: ArticleVisibility,
    pub tags: Vec<String>
}


#[derive(sqlx::Type, Deserialize, Debug)]
#[sqlx(type_name = "article_visibility", rename_all = "PascalCase")]
pub enum ArticleVisibility {
    Public,
    Unlisted,
    Private,
}

impl fmt::Display for ArticleVisibility {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ArticleVisibility::Public => write!(f, "Public"),
            ArticleVisibility::Unlisted => write!(f, "Unlisted"),
            ArticleVisibility::Private => write!(f, "Private"),
        }
    }
}