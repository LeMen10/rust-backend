use serde::{ Serialize, Deserialize };

#[derive(Serialize, Deserialize)]
pub struct Product {
    pub id: String,
    pub categoryId: String,
    pub name: String,
    pub description: String,
    pub thumbnail: String,
    pub minPrice: f64,
    pub maxPrice: f64,
    pub isActive: EStatus,
    pub createdAt: DateTime<Utc>,
    pub updatedAt: DateTime<Utc>,
}