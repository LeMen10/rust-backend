use diesel::Queryable;
use serde::Serialize;

#[derive(Queryable, Serialize)]
#[diesel(table_name = bill)]
pub struct Bill {
    pub id: String,
    pub name: String,
    pub supply_id: Option<String>,
    pub date: Option<chrono::NaiveDateTime>,
    pub total_price: Option<f64>,
    pub is_pay: Option<bool>,
    pub is_delivered: Option<bool>,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
}
