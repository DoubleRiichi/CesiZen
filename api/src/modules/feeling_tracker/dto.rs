use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use crate::modules::feeling::dto::FeelingGet;
use crate::modules::feeling_tracker::model::{ FeelingTrackerWithFeelingRow};

#[derive(Serialize, Debug, ToSchema)]
pub struct FeelingTrackerGet {
    pub id: i32,
    pub user_id: i32,
    pub feeling: String,
    pub feeling_category: String,
    pub timestamp_start: DateTime<Utc>,
    pub timestamp_end: DateTime<Utc>,
    pub intensity: i16,
    pub notes: String,
    pub location: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>
}

impl From<FeelingTrackerWithFeelingRow> for FeelingTrackerGet {
    fn from(row: FeelingTrackerWithFeelingRow) -> Self {
        Self {
            id: row.id,
            user_id: row.user_id,
            feeling: row.feeling_name,
            feeling_category: row.feeling_category_name,
            timestamp_start: row.timestamp_start,
            timestamp_end: row.timestamp_end,
            intensity: row.intensity,
            notes: row.notes,
            location: row.location,
            created_at: row.created_at,
            updated_at: row.updated_at,
        }
    }
}

#[derive(Deserialize, Debug, ToSchema)]
pub struct FeelingTrackerCreate {
    pub user_id: i32,
    pub feeling_id: i32,
    pub timestamp_start: DateTime<Utc>,
    pub timestamp_end: DateTime<Utc>,
    pub intensity: i16,
    pub notes: String,
    pub location: String,
}

#[derive(Deserialize, Debug, ToSchema)]
pub struct FeelingTrackerUpdate {
    pub user_id: i32,
    pub feeling_id: i32,
    pub timestamp_start: DateTime<Utc>,
    pub timestamp_end: DateTime<Utc>,
    pub intensity: i16,
    pub notes: String,
    pub location: String,
}


#[derive(Debug, Deserialize, ToSchema)]
pub struct FeelingTrackerSearchParams {
    pub user_id: Option<i32>,
    pub feeling_id: Option<i32>,
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
    pub cursor: Option<DateTime<Utc>>,
    pub page_size: Option<i32>,
}