use crate::schema::todos;

use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Queryable, Serialize)]
pub struct Todo {
    pub id: Uuid,
    pub title: String,
    pub completed: bool,
    pub created_timestamp: NaiveDateTime,
}

#[derive(Insertable, Clone, Serialize, Deserialize)]
#[table_name = "todos"]
pub struct NewTodo {
    pub title: String,
}
