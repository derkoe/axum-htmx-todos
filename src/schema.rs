// @generated automatically by Diesel CLI.

diesel::table! {
    seaql_migrations (version) {
        version -> Varchar,
        applied_at -> Int8,
    }
}

diesel::table! {
    todos (id) {
        id -> Uuid,
        title -> Varchar,
        completed -> Bool,
        created_timestamp -> Timestamp,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    seaql_migrations,
    todos,
);
