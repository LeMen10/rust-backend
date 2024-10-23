use serde::{Serialize};

#[derive(Serialize)]
pub struct User {
    pub id: String,
    pub username: String,
    pub fullname: String,
    pub password: String,
    pub email: String,
    pub phone_number: String,
    pub role: Role,
    pub is_active: Status,
}

#[derive(Serialize)]
pub enum Role {
    Admin,
    User,
}

#[derive(Serialize)]
pub enum Status {
    Active, 
    InActive,
}
