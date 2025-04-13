use tracing::{debug, info, error};
use axum::{Json, Router, routing::{post, put}, http::StatusCode, extract::State};
use serde::{Deserialize, Serialize};
use axum::response::Result;
use crate::db::user;
use crate::models::user::{User, Email, Password, Name};
use crate::utils::jwt::create_jwt;
use crate::utils::hash_password;
use crate::AppState;

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

/// Auth router
pub fn auth_router() -> Router<AppState> {
    Router::new()
        .route("/signup", post(signup))
        .route("/login", post(login))
        .route("/validate", put(validate))
}

/// Signup handler
async fn signup(State(app_state): State<AppState>,
                Json(signup_request): Json<SignupRequest>)
            ->  Result<Json<SignupResponse>, StatusCode> {

    debug!("Received signup request with payload {:?}", signup_request);

    // check if the name is already in use. Error out if that is the case
    if let Ok(_) = user::get_by_name(&app_state.pool, &signup_request.name).await {
        return Err(StatusCode::CONFLICT); // TODO: response body
    }

    // check if the email address is already in use. Error out if that is the case
    if let Ok(_) = user::get_by_email(&app_state.pool, &signup_request.email).await {
        return Err(StatusCode::CONFLICT); // TODO: response body
    }

    let token = create_jwt(&signup_request.email, false, "your_jwt_secret", 3600)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    debug!("Token: {}", &token);

    let password_hash = hash_password(signup_request.password).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    debug!("Password Hash: {}", &password_hash);

    // Try to create the user in the database
    if let Err(_) = user::create(&app_state.pool, &signup_request.name, &signup_request.email, &password_hash).await {
        return Err(StatusCode::INTERNAL_SERVER_ERROR); // TODO: response body
    }

    // TODO: send validation mail
    
    debug!("User {:?} created, validation mail sent to {:?}", &signup_request.name, &signup_request.email);

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

