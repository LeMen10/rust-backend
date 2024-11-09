use crate::db::config_db::DbPool;
use crate::model::user::CreateUser;
use actix_web::{web, Error, HttpResponse};
use crate::service::user_service::{
    create_new_user, delete_user_service, get_all_users, update_user_service,
};

pub async fn get_users(pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let mut conn = pool.get().map_err(|_| {
        eprintln!("Failed to get DB connection");
        actix_web::error::ErrorInternalServerError("Failed to get DB connection")
    })?;

    match get_all_users(&mut conn) {
        Ok(users) => Ok(HttpResponse::Ok().json(users)),
        Err(e) => {
            eprintln!("Error loading users: {:?}", e);
            Err(actix_web::error::ErrorInternalServerError(
                "Failed to load users",
            ))
        }
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
            Err(actix_web::error::ErrorInternalServerError(
                "Failed to create user",
            ))
        }
    }
}

pub async fn update_user(
    pool: web::Data<DbPool>,
    user_id: web::Path<i32>,
    user_data: web::Json<CreateUser>,
) -> Result<HttpResponse, Error> {
    let mut conn = pool.get().map_err(|_| {
        eprintln!("Failed to get DB connection");
        actix_web::error::ErrorInternalServerError("Failed to get DB connection")
    })?;

    match update_user_service(&mut conn, user_id.into_inner(), &user_data.into_inner()) {
        Ok(_) => Ok(HttpResponse::Ok().body("User  updated successfully")),
        Err(e) => {
            eprintln!("Error updating user: {:?}", e);
            Err(actix_web::error::ErrorInternalServerError(
                "Failed to update user",
            ))
        }
    }
}

pub async fn delete_user(
    pool: web::Data<DbPool>,
    user_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let mut conn = pool.get().map_err(|_| {
        eprintln!("Failed to get DB connection");
        actix_web::error::ErrorInternalServerError("Failed to get DB connection")
    })?;

    match delete_user_service(&mut conn, user_id.into_inner()) {
        Ok(_) => Ok(HttpResponse::Ok().body("User  deleted successfully")),
        Err(e) => {
            eprintln!("Error deleting user: {:?}", e);
            Err(actix_web::error::ErrorInternalServerError(
                "Failed to delete user",
            ))
        }
    }
}
