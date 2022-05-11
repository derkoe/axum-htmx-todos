use axum::response::Redirect;
use axum::routing::get_service;
use axum::Extension;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use chrono::{NaiveDateTime, Utc};
use std::net::SocketAddr;
use tera::Tera;
use tower::ServiceBuilder;
use tower_http::services::ServeDir;
use uuid::Uuid;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Todo {
    pub id: Uuid,
    pub title: String,
    pub completed: bool,
    pub created_timestamp: NaiveDateTime,
}

#[tokio::main]
async fn main() {
    let templates = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*"))
        .expect("Tera initialization failed");
    let app = Router::new()
        .route("/", get(root))
        .route("/todos", get(todos))
        .nest(
            "/static",
            get_service(ServeDir::new(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/static"
            )))
            .handle_error(|error: std::io::Error| async move {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Unhandled internal error: {}", error),
                )
            }),
        )
        .layer(ServiceBuilder::new().layer(Extension(templates)));
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> impl IntoResponse {
    Redirect::to("/todos")
}

async fn todos(
    Extension(ref templates): Extension<Tera>,
) -> Result<Html<String>, (StatusCode, &'static str)> {
    let mut ctx = tera::Context::new();
    let todo = Todo{
        id: Uuid::new_v4(),
        title: "Hello".to_string(),
        completed: false,
        created_timestamp: Utc::now().naive_utc()
    };
    let todos: [Todo; 1] = [todo];

    ctx.insert("todos", &todos);
    let body = templates
        .render("index.html.tera", &ctx)
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Template error"))?;

    Ok(Html(body))
}

// #[derive(Template)]
// #[template(path = "hello.html")]
// struct HelloTemplate {
//     name: String,
// }
