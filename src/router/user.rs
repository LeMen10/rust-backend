use actix_web::web;
use crate::controller::user_controller::{get_users, create_user, update_user, delete_user};

pub fn configure_user_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/user")
            .route("/get-users", web::get().to(get_users))
            .route("/create-user", web::post().to(create_user))
            .route("/update-user/{id}", web::put().to(update_user))
            .route("/delete-user/{id}", web::delete().to(delete_user)) 
    );
}