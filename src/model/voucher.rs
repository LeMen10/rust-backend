use diesel::Queryable;
use serde::{Serialize, Deserialize};
use chrono::NaiveDateTime;
use crate::model::schema::voucher;

#[derive(Queryable, Serialize, Deserialize)]
#[diesel(table_name = voucher)]
pub struct Voucher {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub vouchertype: String,
    pub value: f64,
    pub quantity: i32,
    pub createdat: NaiveDateTime,
    pub updatedat: NaiveDateTime,
}

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = voucher)]
pub struct CreateVoucher {
    pub name: String,
    pub description: String,
    pub vouchertype: String,
    pub value: f64,
    pub quantity: i32
}