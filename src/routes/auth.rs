use sqlx::SqlitePool;
use tracing::error;
use axum::{Json, Router, routing::{post, put}, http::StatusCode, extract::State};
use serde::{Deserialize, Serialize};
use axum::response::Result;
use crate::db::user;
use crate::models::user::{User, Email, Password, Name};
use crate::utils::jwt::create_jwt;
use crate::utils::hash_password;
use tracing::info;
use crate::config::Config;
use std::sync::Arc;

#[derive(Debug, Deserialize)]
struct SignupRequest {
    name: Name,
    email: Email,
    password: Password,
}

#[derive(Debug, Serialize)]
struct SignupResponse {
    message: String,
    token: String,
}

/// Signup route
async fn signup(State(config): State<Arc<Config>>,
                State(pool): State<Arc<SqlitePool>>,
                Json(payload): Json<SignupRequest>)
            ->  Result<Json<SignupResponse>, StatusCode> {
/*
    if let Ok(_) = user::get_by_email(pool, payload.email.clone()).await {
        return Err(StatusCode::CONFLICT);
    }
*/
    let token = create_jwt(&payload.email, false, "your_jwt_secret", 3600)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let password_hash = hash_password(payload.password).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if let Ok(_) = user::create(&pool, &payload.name, &payload.email, &password_hash).await {
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }

    // TODO: send validation mail
    info!("Password Hash: {}", password_hash);

    Ok(Json(SignupResponse {
        message: "User created, validation mail sent".to_string(),
        token,
    }))
}

/// Login route
async fn login() -> Result<&'static str, StatusCode> {
    // TODO: Implement login logic
    Ok("Login successful")
}

/// Validate route
async fn validate() -> Result<&'static str, StatusCode> {
    // TODO: Implement validation logic
    Ok("Validation successful")
}

/// Auth router
pub fn auth_router() -> Router<Arc<Config>> {
    Router::new()
        .route("/signup", post(signup))
        .route("/login", post(login))
        .route("/validate", put(validate))
}