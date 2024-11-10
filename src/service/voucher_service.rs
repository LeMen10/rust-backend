use diesel::prelude::*;
use diesel::PgConnection;
use crate::model::voucher::{Voucher, CreateVoucher};
use crate::model::schema::voucher::dsl::*;

pub fn get_all_voucher(conn: &mut PgConnection) -> QueryResult<Vec<Voucher>> {
    voucher.load::<Voucher>(conn).map_err(|e| {
        eprintln!("Database error: {:?}", e);
        e
    })
}

pub fn create_new_voucher(conn: &mut PgConnection, voucher_data: &CreateVoucher) -> QueryResult<usize> {
    diesel::insert_into(voucher) 
        .values((
            name.eq(&voucher_data.name),
            description.eq(&voucher_data.description),
            vouchertype.eq(&voucher_data.vouchertype),
            value.eq(&voucher_data.value),
            quantity.eq(&voucher_data.quantity),
            createdat.eq(chrono::Utc::now().naive_utc()),
            updatedat.eq(chrono::Utc::now().naive_utc()),
        ))
        .execute(conn)
}

pub fn update_voucher_service(conn: &mut PgConnection, voucher_id: i32, voucher_data: &CreateVoucher) -> QueryResult<usize> {
    let existing_voucher = voucher.find(voucher_id).first::<Voucher>(conn).optional()?;
    if existing_voucher.is_none() { return Err(diesel::result::Error::NotFound);}

    diesel::update(voucher.find(voucher_id))
        .set((
            name.eq(&voucher_data.name),
            description.eq(&voucher_data.description),
            vouchertype.eq(&voucher_data.vouchertype),
            value.eq(&voucher_data.value),
            quantity.eq(&voucher_data.quantity),
            updatedat.eq(chrono::Utc::now().naive_utc()),
        ))
        .execute(conn)
}

pub fn delete_voucher_service(conn: &mut PgConnection, voucher_id: i32) -> QueryResult<usize> {
    diesel::delete(voucher.find(voucher_id)).execute(conn)
}