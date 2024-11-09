use diesel::prelude::*;
use diesel::PgConnection;
use crate::model::user::{User, CreateUser};
use crate::model::schema::users::dsl::*;

pub fn get_all_users(conn: &mut PgConnection) -> QueryResult<Vec<User>> {
    users.load::<User>(conn).map_err(|e| {
        eprintln!("Database error: {:?}", e);
        e
    })
}

pub fn create_new_user(conn: &mut PgConnection, user_data: &CreateUser) -> QueryResult<usize> {
    diesel::insert_into(users)
        .values((
            username.eq(&user_data.username),
            fullname.eq(&user_data.fullname),
            password.eq(&user_data.password),
            email.eq(&user_data.email),
            phonenumber.eq(&user_data.phonenumber),
            role.eq(&user_data.role),
            isactive.eq(&user_data.isactive),
        ))
        .execute(conn)
}

pub fn update_user_service(conn: &mut PgConnection, user_id: i32, user_data: &CreateUser) -> QueryResult<usize> {
    
    let existing_user = users.find(user_id).first::<User>(conn).optional()?;
    if existing_user.is_none() { return Err(diesel::result::Error::NotFound);}

    diesel::update(users.find(user_id))
        .set((
            username.eq(&user_data.username),
            fullname.eq(&user_data.fullname),
            password.eq(&user_data.password),
            email.eq(&user_data.email),
            phonenumber.eq(&user_data.phonenumber),
            role.eq(&user_data.role),
            isactive.eq(&user_data.isactive),
        ))
        .execute(conn)
}

pub fn delete_user_service(conn: &mut PgConnection, user_id: i32) -> QueryResult<usize> {
    diesel::delete(users.find(user_id)).execute(conn)
}