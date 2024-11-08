use diesel::Queryable;
use serde::{Serialize, Deserialize};

use crate::model::schema::users;

#[derive(Queryable, Serialize, Deserialize)]
#[diesel(table_name = users)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub fullname: String,
    pub password: String,
    pub email: String,
    pub phonenumber: String,
    pub role: String,
    pub isactive: String,
}

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = users)]
pub struct CreateUser {
    pub username: String,
    pub fullname: String,
    pub password: String,
    pub email: String,
    pub phonenumber: String,
    #[serde(default = "default_role")]
    pub role: String,
    #[serde(default = "default_isactive")]
    pub isactive: String,
}

fn default_role() -> String {
    "customer".to_string()
}

fn default_isactive() -> String {
    "active".to_string()
}
