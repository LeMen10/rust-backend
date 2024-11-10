use actix_web::{web, App, HttpServer};
use db::config_db::configure_db_pool;
use router::user::configure_user_routes;
use router::product::configure_product_routes;
use router::voucher::configure_voucher_routes;

#[macro_use]
extern crate diesel;

mod model;
mod service;
mod controller;
mod db;
mod router;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = configure_db_pool();
    let app_state = web::Data::new(pool);

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .configure(configure_user_routes)
            .configure(configure_product_routes)
            .configure(configure_voucher_routes)
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}

