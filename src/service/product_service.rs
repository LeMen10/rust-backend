use crate::model::product::{CreateProduct, Product};
use crate::model::schema::product::dsl::*;
use diesel::prelude::*;
use diesel::PgConnection;

pub fn get_all_products(conn: &mut PgConnection) -> QueryResult<Vec<Product>> {
    product.limit(2).load::<Product>(conn).map_err(|e| {
        eprintln!("Database error: {:?}", e);
        e
    })
}

pub fn create_new_product(
    conn: &mut PgConnection,
    product_data: &CreateProduct,
) -> QueryResult<usize> {
    diesel::insert_into(product)
        .values((
            categoryid.eq(product_data.categoryid),
            name.eq(&product_data.name),
            description.eq(&product_data.description),
            thumbnail.eq(&product_data.thumbnail),
            minprice.eq(product_data.minprice),
            maxprice.eq(product_data.maxprice),
            isactive.eq(&product_data.isactive),
            createdat.eq(chrono::Utc::now().naive_utc()),
            updatedat.eq(chrono::Utc::now().naive_utc()),
        ))
        .execute(conn)
}

pub fn update_product_service(
    conn: &mut PgConnection,
    product_id: i32,
    product_data: &CreateProduct,
) -> QueryResult<usize> {
    let existing_product = product.find(product_id).first::<Product>(conn).optional()?;
    if existing_product.is_none() { return Err(diesel::result::Error::NotFound); }
    diesel::update(product.find(product_id))
        .set((
            categoryid.eq(product_data.categoryid),
            name.eq(&product_data.name),
            description.eq(&product_data.description),
            thumbnail.eq(&product_data.thumbnail),
            minprice.eq(product_data.minprice),
            maxprice.eq(product_data.maxprice),
            isactive.eq(&product_data.isactive),
            updatedat.eq(chrono::Utc::now().naive_utc()),
        ))
        .execute(conn)
}

pub fn delete_product_service(conn: &mut PgConnection, product_id: i32) -> QueryResult<usize> {
    diesel::delete(product.find(product_id)).execute(conn)
}
