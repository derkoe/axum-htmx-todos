use axum::response::Redirect;
use axum::routing::get_service;
use axum::Extension;
use axum::{http::StatusCode, response::IntoResponse, routing::get, Router};
use std::net::SocketAddr;
use tera::Tera;
use tower::ServiceBuilder;
use tower_http::services::ServeDir;
mod todos;

#[tokio::main]
async fn main() {
    let templates = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*"))
        .expect("Tera initialization failed");
    let app = Router::new()
        .route("/", get(root))
        .route("/todos", get(todos::web::index))
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
