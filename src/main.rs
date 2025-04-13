use axum::{Router, routing::get};
use sqlx::SqlitePool;
use tracing::{info, debug};
//use tracing_subscriber::filter::EnvFilter;

use anyhow::{Context, Result};
use std::net::SocketAddr;
use crate::config::Config;

mod config;
mod db;
mod models;
mod routes;
mod services;
mod utils;

#[derive(Clone)]
struct AppState {
    pool: SqlitePool,
    config: Config,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing for logging
    // TODO: use env LOG_LEVEL
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_target(false)
        .init();

    // Load configuration and handle errors gracefully
    let config = config::Config::load().context("Failed to load configuration")?;
    info!("Loaded configuration: {:?}", config);

    // Connect to the database and handle errors gracefully
    let pool = db::connect(&config.database_url)
        .await
        .context("Failed to connect to the database")?;
    
    info!("Connected to database");

    // construct the addr to serve from
    let addr = SocketAddr::from(([0, 0, 0, 0], config.port));

    // Build the AppState to share among the routers
    let app_state = AppState { pool, config };

    // Build the application
    let app = Router::new()
        .route("/", get(|| async { "Hello, Quoridor Fan!" }))
        .merge(routes::auth::auth_router()) // Add auth routes
        .merge(routes::user::user_router()) // Add user management routes
        .with_state(app_state);// Pass the database pool to the application state

    // Start the server and handle errors gracefully    
    axum::serve(
        tokio::net::TcpListener::bind(addr).await
            .context("Failed to bind to the specified address and port")?,
        app.into_make_service(),
    )
    .await
    .context("Failed to start the server")?;
    
    info!("Server listening on {}", addr);

    Ok(())
}
