use crate::schema::todos;

use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::Serialize;
use uuid::Uuid;

#[derive(Queryable, Serialize)]
pub struct Todo {
    pub id: Uuid,
    pub title: String,
    pub completed: bool,
    pub created_timestamp: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = todos)]
pub struct NewTodo {
    pub id: Uuid,
    pub title: String,
}
