use self::models::*;
use axum::Form;
use axum::{http::StatusCode, response::Html, Extension};
use axum_htmx_todos::hxrequest::HxRequest;
use axum_htmx_todos::*;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use diesel::{insert_into, prelude::*};
use serde::Deserialize;
use tera::Tera;

pub async fn index(
    Extension(ref templates): Extension<Tera>,
    Extension(ref pool): Extension<Pool<ConnectionManager<PgConnection>>>,
) -> Result<Html<String>, (StatusCode, &'static str)> {
    // let todos = Todo::find().order_by_asc(todo::Column::Id).all(conn).await;

    use self::schema::todos::dsl::*;
    let conn = &mut pool.get().unwrap();
    let todo_list = todos.load::<Todo>(conn).expect("msg");
    let items_left = match todos.filter(completed.eq(false)).count().get_result(conn) {
        Ok(count) => count,
        Err(_) => 0, // TODO better error handling wiht `?`
    };

    let mut ctx = tera::Context::new();
    ctx.insert("todos", &todo_list);
    ctx.insert("items_left", &items_left);

    let body = templates
        .render("index.html.tera", &ctx)
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Template error"))?;

    Ok(Html(body))
}

#[derive(Deserialize)]
pub struct TodoFormData {
    title: String,
}

pub async fn create(
    Form(form_data): Form<TodoFormData>,
    HxRequest(hx_request): HxRequest,
    template_extension: Extension<Tera>,
    dbpool_extension: Extension<Pool<ConnectionManager<PgConnection>>>,
    Extension(ref pool): Extension<Pool<ConnectionManager<PgConnection>>>,
) -> Result<Html<String>, (StatusCode, &'static str)> {
    use schema::todos::dsl::*;
    let todo = NewTodo {
        id: uuid::Uuid::new_v4(),
        title: form_data.title,
    };

    let conn = &mut pool.get().unwrap();

    conn.transaction(|c| insert_into(todos).values(&todo).execute(c))
        .expect("DB Error"); // TODO better error handling

    println!("{}", hx_request);

    index(template_extension, dbpool_extension).await
}
