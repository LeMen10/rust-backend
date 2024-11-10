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

table! {
    product (id) {
        id -> Integer,
        categoryid -> Nullable<Integer>,
        name -> Varchar,
        description -> Nullable<Text>,
        thumbnail -> Nullable<Varchar>,
        minprice -> Double,
        maxprice -> Double,
        isactive -> Varchar,
        createdat -> Timestamp,
        updatedat -> Timestamp,
    }
}

table! {
    categories (id) {
        id -> Integer,
        parent_category_id -> Nullable<Integer>,
        name -> Varchar,
        description -> Nullable<Text>,
        thumbnail -> Nullable<Varchar>,
        status -> Varchar,
        createdat -> Timestamp,
        updatedat -> Timestamp,
    }
}

table! {
    voucher (id) {
        id -> Int4,
        name -> Varchar,
        description -> Varchar,
        vouchertype -> Varchar,
        value -> Float8,
        quantity -> Int4,
        createdat -> Timestamp,
        updatedat -> Timestamp,
    }
}