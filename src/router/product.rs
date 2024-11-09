use actix_web::web;
use crate::controller::product_controller::{
    create_product, delete_product, get_products, update_product,
};

pub fn configure_product_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/product")
            .route("/get-products", web::get().to(get_products))
            .route("/create-product", web::post().to(create_product))
            .route("/update-product/{id}", web::put().to(update_product))
            .route("/delete-product/{id}", web::delete().to(delete_product))
    );
}
