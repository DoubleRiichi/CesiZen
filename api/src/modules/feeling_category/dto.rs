use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use crate::modules::feeling_category::model::FeelingCategoryRow;

#[derive(Serialize, Debug, ToSchema)]
pub struct FeelingCategoryGet {
    pub id: i32,
    pub name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,

}

impl From<(FeelingCategoryRow)> for FeelingCategoryGet {
    fn from(row: FeelingCategoryRow) -> Self {
        Self {
            id: row.id,
            name: row.name,
            created_at: row.created_at,
            updated_at: row.updated_at,
        }
    }
}

#[derive(Deserialize, Debug, ToSchema)]
pub struct FeelingCategoryCreate {
    pub name: String,
}

#[derive(Deserialize, Debug, ToSchema)]
pub struct FeelingCategoryUpdate {
    pub name: String,
}
#[derive(Deserialize, Debug, ToSchema)]
pub struct FeelingCategorySearchParams {
    pub name: Option<String>,
    pub start_at: Option<chrono::NaiveDateTime>,
    pub end_at: Option<chrono::NaiveDateTime>,
    pub cursor: Option<chrono::NaiveDateTime>,
    pub page_size: Option<i32>,
}