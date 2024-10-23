use actix_web::{web, HttpResponse, Responder, get};
use crate::service::user_service::fetch_users;
use deadpool_postgres::Pool;

#[get("/users")]
async fn get_users(pool: web::Data<Pool<tokio_postgres::NoTls>>) -> impl Responder {
    match fetch_users(&pool).await {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error fetching users: {}", e)),
    }
}
