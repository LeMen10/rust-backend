use diesel::r2d2::{ConnectionManager, Pool};
use diesel::pg::PgConnection;
use std::env;
use dotenv::dotenv;

pub type DbPool = Pool<ConnectionManager<PgConnection>>;

pub fn configure_db_pool() -> DbPool {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    println!("DATABASE_URL: {}", database_url);
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder().build(manager).expect("Failed to create pool.")
}
