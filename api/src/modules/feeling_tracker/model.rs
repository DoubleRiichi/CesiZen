use chrono::NaiveDateTime;

#[derive(sqlx::FromRow, Debug)]
pub struct FeelingTracker {
    pub id: i32,
    pub user_id: i32,
    pub feeling_id: i32,
    pub timestamp_start: NaiveDateTime,
    pub timestamp_end: NaiveDateTime,
    pub intensity: i8, //should be between 1 and 10
    pub notes: String, //nullable
    pub location: String, //nullable
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(sqlx::FromRow, Debug)]
pub struct FeelingTrackerWithFeelingRow {
    pub id: i32,
    pub user_id: i32,
    pub feeling_id: i32,
    pub feeling_name: String,
    pub feeling_category_id: i32,
    pub feeling_category_name: String,
    pub timestamp_start: NaiveDateTime,
    pub timestamp_end: NaiveDateTime,
    pub intensity: i8, //should be between 1 and 10
    pub notes: String, //nullable
    pub location: String, //nullable
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,

}