mod config;
mod controllers;
mod db;
mod dto;
mod middlewares;
mod models;
mod routes;
mod schema;
mod services;
mod utils;

use std::sync::Arc;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::config::Config;
use crate::controllers::AppState;
use crate::db::establish_connection_pool;
use crate::routes::create_router;

#[tokio::main]
async fn main() {
    // Initialize logging
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "ecommerce_backend=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load environment variables
    dotenvy::dotenv().ok();

    // Load configuration
    let config = Config::from_env().expect("Failed to load configuration");

    tracing::info!("Starting E-Commerce Backend API");
    tracing::info!("Server: {}:{}", config.server.host, config.server.port);

    // Create database connection pool
    let pool = establish_connection_pool(&config.database.url, config.database.max_connections);
    tracing::info!("Database connection pool established");

    // Create application state
    let state = Arc::new(AppState {
        pool,
        config: config.clone(),
    });

    // Create router with all routes
    let app = create_router(state);

    // Start server
    let addr = format!("{}:{}", config.server.host, config.server.port);
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("Failed to bind to address");

    tracing::info!("Listening on {}", addr);

    axum::serve(listener, app)
        .await
        .expect("Failed to start server");
}
