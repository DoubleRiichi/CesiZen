use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use crate::modules::feeling::model::{ FeelingRow, FeelingWithCategoryRow};
use crate::modules::feeling_category::dto::FeelingCategoryGet;
use crate::modules::user::model::UserRow;

#[derive(Serialize, Debug, ToSchema)]
pub struct FeelingGet {
    pub id: i32,
    pub feeling_category: FeelingCategoryGet,
    pub name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl From<(FeelingWithCategoryRow)> for FeelingGet {
    fn from(row: FeelingWithCategoryRow) -> Self {
        Self {
            id: row.id,
            feeling_category: FeelingCategoryGet {
                id: row.feeling_category_id,
                name: row.feeling_category_name,
                created_at: row.created_at,
                updated_at: row.updated_at,
            },
            name: row.name,
            created_at: row.created_at,
            updated_at: row.updated_at,
        }
    }
}


#[derive(Deserialize, Debug, ToSchema)]
pub struct FeelingCreate {
    pub feeling_category_id: i32,
    pub name: String,
}

#[derive(Deserialize, Debug, ToSchema)]
pub struct FeelingUpdate {
    pub feeling_category_id: i32,
    pub name: String
}

#[derive(Deserialize, Debug, ToSchema)]
pub struct FeelingSearchParams {
    pub name: Option<String>,
    pub feeling_category_id: Option<i32>,
    pub start_at: Option<chrono::NaiveDateTime>,
    pub end_at: Option<chrono::NaiveDateTime>,
    pub cursor: Option<chrono::NaiveDateTime>,
    pub page_size: Option<i32>,
}
