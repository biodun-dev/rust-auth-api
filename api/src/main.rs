mod auth;
mod routes;
mod db;
mod config;
mod middleware;
use axum::{routing::get, Router};
use dotenvy::dotenv;
use std::env;
use tokio::net::TcpListener;
use tracing_subscriber;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use routes::create_router;

// Swagger definition
#[derive(OpenApi)]
#[openapi(
    paths(
        crate::auth::handler::register_user,
        crate::auth::handler::login_user
    ),
    components(schemas(crate::auth::handler::AuthPayload)),
    tags((name = "Auth", description = "User authentication endpoints"))
)]
struct ApiDoc;

#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let app = create_router()
        .route("/", get(|| async { "Welcome to Rust Auth API" }))
        .merge(SwaggerUi::new("/docs").url("/api-doc/openapi.json", ApiDoc::openapi()));

    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let addr = format!("0.0.0.0:{}", port);
    let listener = TcpListener::bind(&addr).await.unwrap();

    println!("🚀 Server running on http://{}", addr);
    println!("📄 Swagger available at http://{}/docs", addr);

    axum::serve(listener, app).await.unwrap();
}
