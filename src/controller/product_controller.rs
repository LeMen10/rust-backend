use crate::db::config_db::DbPool;
use crate::model::product::CreateProduct;
use actix_web::{web, Error, HttpResponse};
use crate::service::product_service::{
    create_new_product, delete_product_service, get_all_products, update_product_service,
};

pub async fn get_products(pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let mut conn = pool.get().map_err(|_| {
        eprintln!("Failed to get DB connection");
        actix_web::error::ErrorInternalServerError("Failed to get DB connection")
    })?;

    match get_all_products(&mut conn) {
        Ok(product) => Ok(HttpResponse::Ok().json(product)),
        Err(e) => {
            eprintln!("Error loading product: {:?}", e);
            Err(actix_web::error::ErrorInternalServerError(
                "Failed to load product",
            ))
        }
    }
}

pub async fn create_product(
    pool: web::Data<DbPool>,
    product_data: web::Json<CreateProduct>,
) -> Result<HttpResponse, Error> {
    let mut conn = pool.get().map_err(|_| {
        eprintln!("Failed to get DB connection");
        actix_web::error::ErrorInternalServerError("Failed to get DB connection")
    })?;

    match create_new_product(&mut conn, &product_data.into_inner()) {
        Ok(_) => Ok(HttpResponse::Created().body("Product created successfully")),
        Err(e) => {
            eprintln!("Error creating product: {:?}", e);
            Err(actix_web::error::ErrorInternalServerError(
                "Failed to create product",
            ))
        }
    }
}

pub async fn update_product(
    pool: web::Data<DbPool>,
    product_id: web::Path<i32>,
    product_data: web::Json<CreateProduct>,
) -> Result<HttpResponse, Error> {
    let mut conn = pool.get().map_err(|_| {
        eprintln!("Failed to get DB connection");
        actix_web::error::ErrorInternalServerError("Failed to get DB connection")
    })?;

    match update_product_service(
        &mut conn,
        product_id.into_inner(),
        &product_data.into_inner(),
    ) {
        Ok(_) => Ok(HttpResponse::Ok().body("Product  updated successfully")),
        Err(e) => {
            eprintln!("Error updating product: {:?}", e);
            Err(actix_web::error::ErrorInternalServerError(
                "Failed to update product",
            ))
        }
    }
}

pub async fn delete_product(
    pool: web::Data<DbPool>,
    product_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let mut conn = pool.get().map_err(|_| {
        eprintln!("Failed to get DB connection");
        actix_web::error::ErrorInternalServerError("Failed to get DB connection")
    })?;

    match delete_product_service(&mut conn, product_id.into_inner()) {
        Ok(_) => Ok(HttpResponse::Ok().body("Product deleted successfully")),
        Err(e) => {
            eprintln!("Error deleting product: {:?}", e);
            Err(actix_web::error::ErrorInternalServerError(
                "Failed to delete product",
            ))
        }
    }
}
