// @generated automatically by Diesel CLI.

diesel::table! {
    todos (id) {
        id -> Uuid,
        title -> Varchar,
        completed -> Bool,
        created_timestamp -> Timestamp,
    }
}
