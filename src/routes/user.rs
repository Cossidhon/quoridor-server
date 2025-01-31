use axum::{Json, Router, routing::{get, put, delete}, http::StatusCode, extract::Path};
use serde::{Deserialize, Serialize};
use anyhow::Result;
use crate::db::user::{db_get_all_users, db_get_user_by_id, db_update_user, db_delete_user};
use crate::models::user::User;

#[derive(Debug, Serialize)]
struct UserResponse {
    userid: String,
    role: String,
    email: String,
}

#[derive(Debug, Deserialize)]
struct UpdateUserRequest {
    role: String,
    email: String,
}

/// Get all users
async fn get_users() -> Result<Json<Vec<UserResponse>>, StatusCode> {
    // TODO: Fetch users from the database
    let users = vec![
        UserResponse {
            userid: "1".to_string(),
            role: "admin".to_string(),
            email: "admin@example.com".to_string(),
        },
    ];

    Ok(Json(users))
}

/// Get a specific user by ID
async fn get_user(Path(userid): Path<String>) -> Result<Json<UserResponse>, StatusCode> {
    // TODO: Fetch user from the database
    let user = UserResponse {
        userid,
        role: "user".to_string(),
        email: "user@example.com".to_string(),
    };

    Ok(Json(user))
}

/// Update a user's information
async fn update_user_handler(
    Path(userid): Path<String>,
    Json(payload): Json<UpdateUserRequest>,
) -> Result<Json<UserResponse>, StatusCode> {
    // TODO: Update user in the database
    let user = UserResponse {
        userid,
        role: payload.role,
        email: payload.email,
    };

    Ok(Json(user))
}

/// Delete a user by ID
async fn delete_user_handler(Path(userid): Path<String>) -> Result<Json<String>, StatusCode> {
    // TODO: Delete user from the database
    Ok(Json(format!("User {} deleted", userid)))
}

/// User management router
pub fn user_router() -> Router {
    Router::new()
        .route("/users", get(get_users))
        .route("/user/:userid", get(get_user).put(update_user_handler).delete(delete_user_handler))
// /        .into()
}