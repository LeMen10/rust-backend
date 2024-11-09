use diesel::{Queryable, Insertable};
use serde::{Serialize, Deserialize};
use crate::model::schema::categories;
use chrono::NaiveDateTime;

#[derive(Queryable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = categories)]
pub struct Category {
    pub id: i32,
    pub parent_category_id: Option<i32>,
    pub name: String,
    pub description: Option<String>,
    pub thumbnail: Option<String>,
    pub status: String,
    pub createdat: NaiveDateTime,
    pub updatedat: NaiveDateTime,
}