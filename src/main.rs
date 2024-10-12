use actix_web::{
    get,
    post,
    put,
    delete,
    web,
    App,
    HttpRequest,
    HttpResponse,
    HttpServer,
    Responder,
    ResponseError,
};
use actix_web::http::header::ContentType;
use actix_web::http::StatusCode;
use actix_web::body::BoxBody;

use serde::{ Serialize, Deserialize };

use std::fmt::Display;
// use std::sync::Mutex;
use deadpool_postgres::{Pool, Client};
mod db;
use db::config_db::configure_db_pool;
mod model;
use model::user::User;



// Implement Responder Trait for Ticket
impl Responder for User {
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        let res_body = serde_json::to_string(&self).unwrap();

        // Create HttpResponse and set Content Type
        HttpResponse::Ok().content_type(ContentType::json()).body(res_body)
    }
}

#[derive(Debug, Serialize)]
struct ErrNoId {
    id: i32,
    err: String,
}

// Implement ResponseError for ErrNoId
impl ResponseError for ErrNoId {
    fn status_code(&self) -> StatusCode {
        StatusCode::NOT_FOUND
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        let body = serde_json::to_string(&self).unwrap();
        let res = HttpResponse::new(self.status_code());
        res.set_body(BoxBody::new(body))
    }
}

// Implement Display for ErrNoId
impl Display for ErrNoId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

struct AppState {
    db_pool: Pool
}


#[post("/add-user")]
async fn post_ticket(data: web::Data<AppState>, user: web::Json<User>) -> impl Responder {
    let client: Client = data.db_pool.get().await.unwrap();

    let stmt = client.prepare("INSERT INTO users (id, author) VALUES ($1, $2)").await.unwrap();

    match client.execute(&stmt, &[&user.id, &user.author]).await {
        Ok(_) => HttpResponse::Created().finish(),
        Err(err) => {
            eprintln!("Error inserting ticket: {:?}", err);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[get("/users")]
async fn get_tickets(data: web::Data<AppState>) -> impl Responder {
    let client: Client = data.db_pool.get().await.unwrap();
    let stmt = client.prepare("SELECT id, author FROM users").await.unwrap();

    let rows = client.query(&stmt, &[]).await.unwrap();

    let tickets: Vec<_> = rows.into_iter().map(|row| {
        serde_json::json!({
            "id": row.get::<_, i32>(0),
            "author": row.get::<_, String>(1),
        })
    }).collect();

    let response = serde_json::to_string(&tickets).unwrap();
    HttpResponse::Ok().content_type(ContentType::json()).body(response)
}

// Get a ticket with the corresponding id
#[get("/user/{id}")]
async fn get_ticket(id: web::Path<i32>, data: web::Data<AppState>) -> Result<HttpResponse, ErrNoId> {
    let client: Client = data.db_pool.get().await.unwrap();
    let stmt = client.prepare("SELECT id, author FROM users WHERE id = $1").await.unwrap();

    let rows = client.query(&stmt, &[&*id]).await.unwrap();

    if let Some(row) = rows.get(0) {
        let ticket = serde_json::json!({
            "id": row.get::<_, i32>(0),
            "author": row.get::<_, String>(1),
        });
        let response = serde_json::to_string(&ticket).unwrap();
        Ok(HttpResponse::Ok().content_type(ContentType::json()).body(response))
    } else {
        Err(ErrNoId {
            id: *id,
            err: String::from("ticket not found"),
        })
    }
}

// Update the ticket with the corresponding id
#[put("/user/{id}")]
async fn update_ticket(id: web::Path<i32>, data: web::Data<AppState>) -> Result<HttpResponse, ErrNoId> {
    let client: Client = data.db_pool.get().await.unwrap();

    let stmt = client.prepare("UPDATE users SET author = $1 WHERE id = $2").await.unwrap();
    let result = client.execute(&stmt, &[&"Updated Author", &*id]).await.unwrap();

    if result > 0 {
        let updated_ticket = serde_json::json!({
            "id": *id,
            "author": "Updated Author",
        });
        let response = serde_json::to_string(&updated_ticket).unwrap();
        Ok(HttpResponse::Ok().content_type(ContentType::json()).body(response))
    } else {
        Err(ErrNoId {
            id: *id,
            err: String::from("ticket not found"),
        })
    }
}

// Delete the ticket with the corresponding id
#[delete("/user/{id}")]
async fn delete_ticket(id: web::Path<i32>, data: web::Data<AppState>) -> Result<HttpResponse, ErrNoId> {
    let client: Client = data.db_pool.get().await.unwrap();

    let stmt = client.prepare("DELETE FROM users WHERE id = $1 RETURNING id, author").await.unwrap();
    let rows = client.query(&stmt, &[&*id]).await.unwrap();

    if let Some(row) = rows.get(0) {
        let deleted_ticket = serde_json::json!({
            "id": row.get::<_, i32>(0),
            "author": row.get::<_, String>(1),
        });
        let response = serde_json::to_string(&deleted_ticket).unwrap();
        Ok(HttpResponse::Ok().content_type(ContentType::json()).body(response))
    } else {
        Err(ErrNoId {
            id: *id,
            err: String::from("ticket not found"),
        })
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = configure_db_pool().await;
    let app_state = web::Data::new(AppState {
        db_pool: pool
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .service(post_ticket)
            .service(get_ticket)
            .service(get_tickets)
            .service(update_ticket)
            .service(delete_ticket)
    })
        .bind(("127.0.0.1", 8000))?
        .run().await
}