use actix_web::web;
use crate::controller::voucher_controller::{get_vouchers, create_voucher, update_voucher, delete_voucher};

pub fn configure_voucher_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/voucher")
            .route("/get-vouchers", web::get().to(get_vouchers))
            .route("/create-voucher", web::post().to(create_voucher))
            .route("/update-voucher/{id}", web::put().to(update_voucher))
            .route("/delete-voucher/{id}", web::delete().to(delete_voucher)) 
    );
}