use actix_web::{App, HttpServer, web};
mod controller;
mod model;
mod router;
mod service;
mod db;
use router::user::configure;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = db::config_db::configure_db_pool().await;
    HttpServer::new(move || {
        App::new()
                .app_data(web::Data::new(pool.clone()))
                .configure(configure) 
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}