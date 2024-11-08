use actix_web::web;
use crate::controller::user_controller::{get_users, create_user};

pub fn configure_user_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .route("/users", web::get().to(get_users))
            .route("/create-user", web::post().to(create_user))
    );
}