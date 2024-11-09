use diesel::Queryable;
use serde::{Serialize, Deserialize};
use crate::model::schema::product;
use chrono::NaiveDateTime;

#[derive(Queryable, Serialize, Deserialize)]
#[diesel(table_name = product)]
pub struct Product {
    pub id: i32,
    pub categoryid: Option<i32>,
    pub name: String,
    pub description: Option<String>,
    pub thumbnail: Option<String>,
    pub minprice: f64,
    pub maxprice: f64,
    pub isactive: String,
    pub createdat: NaiveDateTime,
    pub updatedat: NaiveDateTime,
}

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = product)]
pub struct CreateProduct {
    pub categoryid: Option<i32>,
    pub name: String,
    pub description: Option<String>,
    pub thumbnail: Option<String>,
    pub minprice: f64,
    pub maxprice: f64,
    #[serde(default = "default_isactive")]
    pub isactive: String,
}

fn default_isactive() -> String {
    "active".to_string()
}
