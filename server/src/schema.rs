// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Int4,
        first_name -> Varchar,
        last_name -> Varchar,
        email -> Varchar,
        username -> Varchar,
        password -> Varchar,
        token -> Nullable<Varchar>,
    }
}
