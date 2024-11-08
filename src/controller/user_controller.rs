use actix_web::{web, HttpResponse, Error};
use crate::service::user_service::{get_all_users, create_new_user};
use crate::db::config_db::DbPool;
use crate::model::user::CreateUser;

pub async fn get_users(pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let mut conn = pool.get().map_err(|_| {
        eprintln!("Failed to get DB connection");
        actix_web::error::ErrorInternalServerError("Failed to get DB connection")
    })?;

    match get_all_users(&mut conn) {
        Ok(users) => Ok(HttpResponse::Ok().json(users)),
        Err(e) => {
            eprintln!("Error loading users: {:?}", e);
            Err(actix_web::error::ErrorInternalServerError("Failed to load users"))
        },
    }
}

pub async fn create_user(
    pool: web::Data<DbPool>,
    user_data: web::Json<CreateUser>,
) -> Result<HttpResponse, Error> {
    let mut conn = pool.get().map_err(|_| {
        eprintln!("Failed to get DB connection");
        actix_web::error::ErrorInternalServerError("Failed to get DB connection")
    })?;

    match create_new_user(&mut conn, &user_data.into_inner()) {
        Ok(_) => Ok(HttpResponse::Created().body("User  created successfully")),
        Err(e) => {
            eprintln!("Error creating user: {:?}", e);
            Err(actix_web::error::ErrorInternalServerError("Failed to create user"))
        },
    }
}