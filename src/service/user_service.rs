use deadpool_postgres::{ Pool, PoolError };
use crate::model::user::{ User, Role, Status };

pub async fn fetch_users(pool: &Pool<tokio_postgres::NoTls>) -> Result<Vec<User>, PoolError> {
    let client = pool.get().await?;
    let rows = client.query(
        "SELECT id, username, fullname, password, email, phoneNumber, role::text, isActive::text FROM users",
        &[]
    ).await?;

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
                _ => panic!("Unknown status: {}", row.get::<_, String>(7)),
            },
        };
        results.push(user);
    }

    Ok(results)
}
