use crate::modules::tag::model::TagRow;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Debug, ToSchema)]
pub struct TagGet {
    pub id: i32,
    pub name: String,
}


impl From<TagRow> for TagGet {
    fn from(row: TagRow) -> Self {
        Self {
            id: row.id,
            name: row.name,
        }
    }
}

#[derive(Deserialize, Debug, ToSchema)]
pub struct TagCreate {
    pub name: String,
}

#[derive(Deserialize, Debug, ToSchema)]
pub struct TagUpdate {
    pub name: String,
}