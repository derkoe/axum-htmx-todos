use axum::response::Redirect;
use axum::routing::{get_service, post};
use axum::Extension;
use axum::{http::StatusCode, response::IntoResponse, routing::get, Router};
use migration::{Migrator, MigratorTrait};
use sea_orm::{prelude::*, Database};
use std::{env, net::SocketAddr};
use tera::Tera;
use tower::ServiceBuilder;
use tower_http::services::ServeDir;
mod todos;

#[tokio::main]
async fn main() {
    let db_conn = db_conn().await.expect("Bla");
    Migrator::up(&db_conn, None).await.unwrap();

    let templates = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*"))
        .expect("Tera initialization failed");
    let app = Router::new()
        .route("/", get(root))
        .route("/todos", get(todos::web::index))
        .route("/todos", post(todos::web::create))
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
        .layer(
            ServiceBuilder::new()
                .layer(Extension(templates))
                .layer(Extension(db_conn)),
        );
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

async fn db_conn() -> Result<DatabaseConnection, DbErr> {
    dotenv::dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    Database::connect(db_url).await
}
