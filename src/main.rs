use axum::{Router, routing::get, extract::State};
use tracing::info;
use anyhow::{Context, Result};
use std::net::SocketAddr;
use std::sync::Arc;

mod config;
mod db;
mod models;
mod routes;
mod services;
mod utils;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing for logging
    tracing_subscriber::fmt::init();

    // Load configuration and handle errors gracefully
    let config = config::Config::load().context("Failed to load configuration")?;
    info!("Loaded configuration: {:?}", config);

    // Connect to the database and handle errors gracefully
    let pool = db::connect(&config.database_url)
        .await
        .context("Failed to connect to the database")?;
    info!("Connected to database");

    // Save port to a variable before config is moved
    let port = config.port;

    // Build the application
    let app = Router::new()
        .route("/", get(|| async { "Hello, Quoridor!" }))
        .merge(routes::auth::auth_router()) // Add auth routes
        .merge(routes::user::user_router()) // Add user management routes
        .with_state(Arc::new(config)) // Pass the configuration to the application state
        .with_state(Arc::new(pool)); // Pass the database pool to the application state

    // Start the server and handle errors gracefully
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    info!("Server listening on {}", addr);

    axum::serve(
        tokio::net::TcpListener::bind(addr).await
            .context("Failed to bind to the specified address and port")?,
        app.into_make_service(),
    )
    .await
    .context("Failed to start the server")?;

    Ok(())
}
