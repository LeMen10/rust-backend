use actix_web::web;
use crate::controller::user_controller::get_users;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(get_users);
}
