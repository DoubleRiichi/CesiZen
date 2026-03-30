use chrono::{DateTime, Utc};

#[derive(sqlx::FromRow, Debug)]
pub struct FeelingCategoryRow {
    pub id: i32,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}