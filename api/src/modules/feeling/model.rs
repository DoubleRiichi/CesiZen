use chrono::{DateTime, Utc};
use sqlx::FromRow;

#[derive(FromRow, Debug)]
pub struct FeelingRow {
    pub id: i32,
    pub feeling_category_id: i32,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}


#[derive(sqlx::FromRow, Debug)]
pub struct FeelingWithCategoryRow {
    pub id: i32,
    pub feeling_category_id: i32,
    pub feeling_category_name: String,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}