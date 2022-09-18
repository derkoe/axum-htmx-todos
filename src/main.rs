use axum::response::Redirect;
use axum::routing::{get_service, post};
use axum::Extension;
use axum::{http::StatusCode, response::IntoResponse, routing::get, Router};
use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;
use std::{env, net::SocketAddr};
use tera::Tera;
use tower::ServiceBuilder;
use tower_http::services::ServeDir;
mod web;

#[tokio::main]
async fn main() {
    let pool = get_connection_pool();
    let templates = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*"))
        .expect("Tera initialization failed");

    let app = Router::new()
        .route("/", get(root))
        .route("/todos", get(web::index))
        .route("/todos", post(web::create))
        .route("/todos/:id/delete", post(web::delete))
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
                .layer(Extension(pool)),
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

fn get_connection_pool() -> Pool<ConnectionManager<PgConnection>> {
    dotenvy::dotenv().ok();
    let url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let manager = ConnectionManager::<PgConnection>::new(url);
    Pool::builder()
        .test_on_check_out(true)
        .build(manager)
        .expect("Could not build connection pool")
}

// impl From<diesel::result::Error> for axum::http::StatusCode {
//     fn from(_: diesel::result::Error) -> Self {
//         StatusCode::INTERNAL_SERVER_ERROR
//     }
// }
