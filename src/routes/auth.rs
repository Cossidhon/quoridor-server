use axum::{Json, Router, routing::post, http::StatusCode};
use serde::{Deserialize, Serialize};
use axum::response::Result;
use crate::db::user::{db_create_user, db_get_user_by_email};
use crate::models::user::User;
use crate::utils::jwt::create_jwt;

#[derive(Debug, Deserialize)]
struct SignupRequest {
    email: String,
    password: String,
    role: String,
}

#[derive(Debug, Serialize)]
struct SignupResponse {
    message: String,
    token: String,
}

/// Signup route
async fn signup(Json(payload): Json<SignupRequest>) -> Result<Json<SignupResponse>, StatusCode> {
    // TODO: Validate input, hash password, and create user
    let token = create_jwt(&payload.email, &payload.role, "your_jwt_secret", 3600)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(SignupResponse {
        message: "User created successfully".to_string(),
        token,
    }))
}

/// Login route
async fn login() -> Result<&'static str, StatusCode> {
    // TODO: Implement login logic
    Ok("Login successful")
}

/// Auth router
pub fn auth_router() -> Router {
    Router::new()
        .route("/signup", post(signup))
        .route("/login", post(login))
}