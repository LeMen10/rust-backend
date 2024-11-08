use diesel::Queryable;
use serde::Serialize;

#[derive(Queryable, Serialize)]
#[diesel(table_name = product)]
pub struct Product {
    pub id: String,
    pub category_id: Option<String>,
    pub name: String,
    pub description: Option<String>,
    pub thumbnail: Option<String>,
    pub min_price: Option<f64>,
    pub max_price: Option<f64>,
    pub is_active: Option<String>,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
}
