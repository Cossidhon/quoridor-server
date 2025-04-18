use axum::{Router, routing::get};
use sqlx::SqlitePool;
use tracing::{info, debug};
use anyhow::{anyhow, Context, Result};
use std::env;
use crate::utils::init_check;
use std::net::SocketAddr;
use crate::config::Config;

mod config;
mod db;
mod models;
mod handlers;
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

    // database url
    let database_url = env::var("DATABASE_URL")
        .map_err(|_| anyhow!("DATABASE_URL is required and must be a valid SQLite connection string"))?;

    // server port
    let port = env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string()) // Default to 3000 if not provided
        .parse::<u16>()
        .map_err(|_| anyhow!("PORT must be a valid number between 1 and 65535"))?;

    // Connect to the database and handle errors gracefully
    let pool = db::connect(&database_url)
        .await
        .context("Failed to connect to the database")?;
    
    info!("Connected to database");

    // Build the AppState to share among the routers
    let app_state = AppState { pool, config };

    // Check for initial conditions that might apply
    let _ = init_check(&app_state).await;
    
    // construct the address (ip:port) to serve from
    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    // Build the application
    let app = Router::new()
        .route("/", get(|| async { "Hello, Quoridor Fan!" }))
        .merge(handlers::auth::auth_router()) // Add auth routes
        .merge(handlers::user::user_router()) // Add user management routes
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
