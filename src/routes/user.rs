use axum::{Json, Router, routing::{get, put, delete}, http::StatusCode, extract::Path};
use serde::{Deserialize, Serialize};
use anyhow::Result;
use crate::{db::user, AppState};
use crate::models::user::{User, Email, Password};
use crate::config::Config;

#[derive(Debug, Serialize)]
struct UserResponse {
    email: Email,
    is_admin: bool,
    is_valid: bool,
    is_active: bool,
}

#[derive(Debug, Deserialize)]
struct ChangeEmailRequest {
    password: Password,
    new_email: Email,
}

#[derive(Debug, Deserialize)]
struct ChangePasswordRequest {
    password: Password,
    new_password: Password
}

/// User management router
pub fn user_router() -> Router<AppState> {
    Router::new()
        .route("/users", get(get_users))
        .route("/user/:email", 
            get(get_user)
            .put(update_user_handler)
            .delete(delete_user_handler)
            .post(create_user_handler))
        .route("/user/:email/password", put(change_password))
        .route("/user/:email/email", put(change_email))
}

/// Get all users
async fn get_users() -> Result<Json<Vec<UserResponse>>, StatusCode> {
    // TODO: Fetch users from the database
    let users = vec![
        UserResponse {
            email: Email::new("user@example.com").unwrap(),
            is_admin: false,
            is_valid: false,
            is_active: false,
        },
    ];

    Ok(Json(users))
}

/// Get a user
async fn get_user(Path(email): Path<Email>) -> Result<Json<UserResponse>, StatusCode> {
    // TODO: Fetch user from the database
    let user = UserResponse {
        email: email,
        is_admin: true,
        is_valid: true,
        is_active: true,
    };

    Ok(Json(user))
}

/// Change a user
async fn update_user_handler(
    Path(email): Path<String>,
    Json(payload): Json<User>,
) -> Result<Json<String>, StatusCode> {
    // TODO: Update user in the database
    Ok(Json(format!("User {} updated", email)))
}

/// Delete a user
/// Password required for extra security
async fn delete_user_handler(Path(email): Path<String>) -> Result<Json<String>, StatusCode> {
    // TODO: Delete user from the database
    Ok(Json(format!("User {} deleted", email)))
}

/// Create a user
async fn create_user_handler(Json(payload): Json<User>) -> Result<Json<String>, StatusCode> {
    // TODO: Create user in the database
    Ok(Json(format!("User {} created", payload.email)))
}

/// Change a user's password
/// Password required for extra security
async fn change_password(Path(email): Path<String>, Json(payload): Json<ChangePasswordRequest>) -> Result<Json<String>, StatusCode> {
    // TODO: Change password in the database
    Ok(Json(format!("Password changed, relogin with the new password")))
}

/// Change a user's email
/// Password required for extra security
async fn change_email(Path(email): Path<String>, Json(payload): Json<ChangeEmailRequest>) -> Result<Json<String>, StatusCode> {
    // TODO: Change email in the database
    Ok(Json(format!("Email changed, login with the new email")))
}

