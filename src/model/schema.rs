use diesel::table;

table! {
    users (id) {
        id -> Integer,
        username -> Varchar,
        fullname -> Varchar,
        password -> Varchar,
        email -> Varchar,
        phonenumber -> Varchar,
        role -> Varchar,
        isactive -> Varchar,
    }
}
