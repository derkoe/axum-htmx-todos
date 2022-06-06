use axum::extract::Form;
use axum::{http::StatusCode, response::Html, Extension};
use entity::todo;
use entity::todo::Entity as Todo;
use sea_orm::DatabaseConnection;
use sea_orm::{prelude::*, QueryOrder, Set};
use tera::Tera;

pub async fn index(
    Extension(ref templates): Extension<Tera>,
    Extension(ref conn): Extension<DatabaseConnection>,
) -> Result<Html<String>, (StatusCode, &'static str)> {
    let todos = Todo::find().order_by_asc(todo::Column::Id).all(conn).await;

    let mut ctx = tera::Context::new();
    ctx.insert("todos", &todos.unwrap());
    ctx.insert("items_left", &1);

    let body = templates
        .render("index.html.tera", &ctx)
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Template error"))?;

    Ok(Html(body))
}

pub async fn create(
    template_extension: Extension<Tera>,
    db_extension: Extension<DatabaseConnection>,
    Extension(ref conn): Extension<DatabaseConnection>,
    form: Form<todo::Model>,
) -> Result<Html<String>, (StatusCode, &'static str)> {
    let model = form.0;

    todo::ActiveModel {
        title: Set(model.title),
        ..Default::default()
    }
    .save(conn)
    .await
    .expect("Could not save todo");

    return index(template_extension, db_extension).await;
}
