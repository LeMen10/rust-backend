use deadpool_postgres::{Config, Pool, RecyclingMethod, Runtime};
use tokio_postgres::NoTls;

pub async fn configure_db_pool() -> Pool {
    let mut cfg = Config::new();
    cfg.dbname = Some("rust-ecommerce".to_string());
    cfg.user = Some("postgres".to_string());
    cfg.password = Some("1".to_string());
    cfg.host = Some("localhost".to_string());
    cfg.port = Some(5432);

    cfg.manager = Some(deadpool_postgres::ManagerConfig {
        recycling_method: RecyclingMethod::Fast,
    });

    cfg.create_pool(Some(Runtime::Tokio1), NoTls).unwrap()
}
