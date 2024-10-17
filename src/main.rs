use actix_web::{web, App, HttpResponse, HttpServer, Responder, get};
use deadpool_postgres::{Pool, Config, PoolError};
use serde::{Serialize};
use tokio_postgres::NoTls;

#[derive(Serialize)]
struct User {
    id: String,
    username: String,
    fullname: String,
    password: String,
    email: String,
    phone_number: String,
    role: Role,
    is_active: Status,
}

#[derive(Serialize)]
enum Role {
    Admin,
    User
}

#[derive(Serialize)]
enum Status {
    Active, 
    InActive
}

async fn fetch_users(pool: &Pool<NoTls>) -> Result<Vec<User>, PoolError> {
    let client = pool.get().await?;
    let rows = client.query("SELECT id, username, fullname, password, email, phoneNumber, role::text, isActive::text FROM users", &[]).await?;

    let mut results = Vec::new();
    for row in rows {
        let user = User {
            id: row.get(0),
            username: row.get(1),
            fullname: row.get(2),
            password: row.get(3),
            email: row.get(4),
            phone_number: row.get(5),
            role: match row.get::<_, String>(6).as_str() {
                "admin" => Role::Admin,
                "user" => Role::User,
                _ => panic!("Unknown role: {}", row.get::<_, String>(6)),
            },
            is_active: match row.get::<_, String>(7).as_str() {
                "active" => Status::Active,
                "inActive" => Status::InActive,
                _ => panic!("Unknown role: {}", row.get::<_, String>(7)),
            },
        };
        results.push(user);
    }

    Ok(results)
}

#[get("/users")]
async fn get_users(pool: web::Data<Pool<NoTls>>) -> impl Responder {
    match fetch_users(&pool).await {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error fetching users: {}", e)),
    }
}

async fn configure_db_pool() -> Pool<NoTls> {
    let mut cfg = Config::new();
    cfg.dbname = Some("rust-ecommerce".to_string());
    cfg.user = Some("postgres".to_string());
    cfg.password = Some("1".to_string());
    cfg.host = Some("localhost".to_string());

    cfg.create_pool(NoTls).unwrap()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = configure_db_pool().await;
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(get_users)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}