use crate::todos::model::*;
use axum::{http::StatusCode, response::Html, Extension};
use chrono::Utc;
use tera::Tera;
use uuid::Uuid;

pub async fn index(
    Extension(ref templates): Extension<Tera>,
) -> Result<Html<String>, (StatusCode, &'static str)> {
    let mut ctx = tera::Context::new();
    let todo = Todo {
        id: Uuid::new_v4(),
        title: "Hello".to_string(),
        completed: false,
        created_timestamp: Utc::now().naive_utc(),
    };
    let todos: [Todo; 1] = [todo];

    ctx.insert("todos", &todos);
    ctx.insert("items_left", &1);

    let body = templates
        .render("index.html.tera", &ctx)
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Template error"))?;

    Ok(Html(body))
}
