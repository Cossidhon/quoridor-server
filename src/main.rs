use axum::{Router, routing::get};
use tracing::info;
use anyhow::{Context, Result};
use std::net::SocketAddr;

mod config;
mod db;
mod models;
mod routes;
mod services;
mod utils;

#[tokio::main]
async fn main() -> Result<()> {
    // Disable back tracing by default
    std::env::set_var("RUST_BACKTRACE", "0");

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

    // Build the application
    let app = Router::new().route("/", get(|| async { "Hello, Quoridor!" }));

    // Start the server and handle errors gracefully
    let addr = SocketAddr::from(([0, 0, 0, 0], config.port));
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