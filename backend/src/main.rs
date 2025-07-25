use axum::Router;
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;
use tracing::info;

mod config;
mod db;
mod error;
mod models;
mod routes;

use crate::error::AppError;

#[tokio::main]
async fn main() -> Result<(), AppError> {
    // Initialize tracing subscriber
    tracing_subscriber::fmt::init();

    // Load environment variables
    dotenvy::dotenv().ok();

    // Get database URL from environment
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // Create database connection pool
    let pool = db::create_pool(&database_url).await?;

    // Run database migrations
    db::run_migrations(&pool).await?;

    // Build our application with routes
    let app = Router::new()
        .nest("/api", routes::create_router())
        .route("/health", axum::routing::get(health_check))
        .layer(
            CorsLayer::new()
                .allow_origin(tower_http::cors::Any)
                .allow_methods([
                    axum::http::Method::GET,
                    axum::http::Method::POST,
                    axum::http::Method::OPTIONS,
                ])
                .allow_headers(tower_http::cors::Any),
        )
        .with_state(pool);

    // Define the address to bind to
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));

    info!("Backend listening on http://{}", addr);

    // Create TCP listener and serve the app
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

async fn health_check() -> &'static str {
    "OK"
}
