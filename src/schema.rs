// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
        surname -> Varchar,
        age -> Int4,
    }
}
