use self::models::*;
use axum::extract::Path;
use axum::http::Response;
use axum::response::Redirect;
use axum::Form;
use axum::{http::StatusCode, Extension};
use axum_htmx_todos::hxrequest::HxRequest;
use axum_htmx_todos::*;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use diesel::{insert_into, prelude::*};
use serde::Deserialize;
use tera::Tera;
use uuid::Uuid;

pub async fn index(
    Extension(ref templates): Extension<Tera>,
    Extension(ref pool): Extension<Pool<ConnectionManager<PgConnection>>>,
) -> Response<String> {
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
        .expect("Template Error");

    Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "text/html")
        .body(body)
        .unwrap()
}

async fn item(Extension(ref templates): Extension<Tera>, todo: Todo) -> Response<String> {
    let mut ctx = tera::Context::new();
    ctx.insert("todo", &todo);

    let body = templates
        .render("item.html.tera", &ctx)
        .expect("Template Error");

    Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "text/html")
        .header("HX-Trigger", "clear-add-todo")
        .body(body)
        .unwrap()
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
) -> Response<String> {
    use schema::todos::dsl::*;
    let todo = NewTodo {
        id: uuid::Uuid::new_v4(),
        title: form_data.title,
    };

    let conn = &mut pool.get().unwrap();

    conn.transaction(|c| insert_into(todos).values(&todo).execute(c))
        .expect("DB Error"); // TODO better error handling

    let created_todo: Todo = todos.filter(id.eq(todo.id)).first(conn).unwrap();

    if hx_request {
        item(template_extension, created_todo).await
    } else {
        index(template_extension, dbpool_extension).await
    }
}

pub async fn delete(
    Path(id): Path<Uuid>,
    Extension(ref pool): Extension<Pool<ConnectionManager<PgConnection>>>,
) -> Redirect {
    use schema::todos::dsl::id as id_col;
    use schema::todos::dsl::todos;
    diesel::delete(todos.filter(id_col.eq(id)))
        .execute(&mut pool.get().unwrap())
        .expect("DB Error");

    Redirect::to("/todos")
}
