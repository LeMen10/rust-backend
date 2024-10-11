pub mod db {
    use sqlx::{PgPool, postgres::PgPoolOptions};

    pub async fn create_pool() -> PgPool {
        PgPoolOptions::new()
            .max_connections(5)
            .connect("postgres://postgres:1@localhost/rust-db")
            .await
            .expect("Unable to connect to database")
    }
}
