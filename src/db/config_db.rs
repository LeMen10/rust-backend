use deadpool_postgres::{Config, Pool};
use tokio_postgres::NoTls;

pub async fn configure_db_pool() -> Pool<NoTls> {
    let mut cfg = Config::new();
    cfg.dbname = Some("rust-ecommerce".to_string());
    cfg.user = Some("postgres".to_string());
    cfg.password = Some("1".to_string());
    cfg.host = Some("localhost".to_string());

    cfg.create_pool(NoTls).unwrap()
}
