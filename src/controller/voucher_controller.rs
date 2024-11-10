use crate::db::config_db::DbPool;
use crate::model::voucher::CreateVoucher;
use actix_web::{web, Error, HttpResponse};
use crate::service::voucher_service::{
    create_new_voucher, delete_voucher_service, get_all_voucher, update_voucher_service,
};

pub async fn get_vouchers(pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let mut conn = pool.get().map_err(|_| {
        eprintln!("Failed to get DB connection");
        actix_web::error::ErrorInternalServerError("Failed to get DB connection")
    })?;

    match get_all_voucher(&mut conn) {
        Ok(voucher) => Ok(HttpResponse::Ok().json(voucher)),
        Err(e) => {
            eprintln!("Error loading voucher: {:?}", e);
            Err(actix_web::error::ErrorInternalServerError(
                "Failed to load voucher",
            ))
        }
    }
}

pub async fn create_voucher(
    pool: web::Data<DbPool>,
    voucher_data: web::Json<CreateVoucher>,
) -> Result<HttpResponse, Error> {
    let mut conn = pool.get().map_err(|_| {
        eprintln!("Failed to get DB connection");
        actix_web::error::ErrorInternalServerError("Failed to get DB connection")
    })?;

    match create_new_voucher(&mut conn, &voucher_data.into_inner()) {
        Ok(_) => Ok(HttpResponse::Created().body("Voucher  created successfully")),
        Err(e) => {
            eprintln!("Error creating voucher: {:?}", e);
            Err(actix_web::error::ErrorInternalServerError(
                "Failed to create voucher",
            ))
        }
    }
}

pub async fn update_voucher(
    pool: web::Data<DbPool>,
    voucher_id: web::Path<i32>,
    voucher_data: web::Json<CreateVoucher>,
) -> Result<HttpResponse, Error> {
    let mut conn = pool.get().map_err(|_| {
        eprintln!("Failed to get DB connection");
        actix_web::error::ErrorInternalServerError("Failed to get DB connection")
    })?;

    match update_voucher_service(&mut conn, voucher_id.into_inner(), &voucher_data.into_inner()) {
        Ok(_) => Ok(HttpResponse::Ok().body("Voucher  updated successfully")),
        Err(e) => {
            eprintln!("Error updating voucher: {:?}", e);
            Err(actix_web::error::ErrorInternalServerError(
                "Failed to update voucher",
            ))
        }
    }
}

pub async fn delete_voucher(
    pool: web::Data<DbPool>,
    delete_voucher_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let mut conn = pool.get().map_err(|_| {
        eprintln!("Failed to get DB connection");
        actix_web::error::ErrorInternalServerError("Failed to get DB connection")
    })?;

    match delete_voucher_service(&mut conn, delete_voucher_id.into_inner()) {
        Ok(_) => Ok(HttpResponse::Ok().body("Delete_voucher  deleted successfully")),
        Err(e) => {
            eprintln!("Error deleting delete_voucher: {:?}", e);
            Err(actix_web::error::ErrorInternalServerError(
                "Failed to delete delete_voucher",
            ))
        }
    }
}
