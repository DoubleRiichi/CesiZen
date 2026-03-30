use chrono::{DateTime, Utc};

#[derive(sqlx::FromRow, Debug)]
pub struct FeelingTracker {
    pub id: i32,
    pub user_id: i32,
    pub feeling_id: i32,
    pub timestamp_start: DateTime<Utc>,
    pub timestamp_end: DateTime<Utc>,
    pub intensity: i16, //should be between 1 and 10
    pub notes: String, //nullable
    pub location: String, //nullable
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(sqlx::FromRow, Debug)]
pub struct FeelingTrackerWithFeelingRow {
    pub id: i32,
    pub user_id: i32,
    pub feeling_id: i32,
    pub feeling_name: String,
    pub feeling_category_id: i32,
    pub feeling_category_name: String,
    pub timestamp_start: DateTime<Utc>,
    pub timestamp_end: DateTime<Utc>,
    pub intensity: i16,
    pub notes: String, //nullable
    pub location: String, //nullable
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,

}