use serde::{Serialize};

#[derive(Serialize)]
struct User {
    id: String,
    username: String,
    fullname: String,
    password: String,
    email: String,
    phone_number: String,
    role: String,
    is_active: String,
}