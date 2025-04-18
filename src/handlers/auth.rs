use tracing::{debug, info, error};
use axum::{Json, Router, routing::{post, put}, http::StatusCode, extract::State};
use serde::{Deserialize, Serialize};
use axum::response::Result;
use crate::{db::user, models::user::ValidationCode};
use crate::models::user::{Email, Password, Name};   // todo: convert to User
use crate::utils::jwt::create_jwt;
use crate::utils::hash_password;
use crate::AppState;
use crate::utils;

#[derive(Debug, Deserialize)]
struct SignupRequest {
    name: Name,
    email: Email,
    password: Password,
}

#[derive(Debug, Serialize)]
struct SignupResponse {
    message: String,
    bearer: String,
}

/// Auth router
pub fn auth_router() -> Router<AppState> {
    Router::new()
        .route("/signup", post(signup))
        .route("/validate", put(validate))
        .route("/login", post(login))
}

/// Signup handler
async fn signup(State(app_state): State<AppState>,
                Json(signup_request): Json<SignupRequest>)
            ->  Result<(StatusCode, String), (StatusCode, String)> {

    debug!("Received signup request with payload {:?}", signup_request);

    // check if the name is already in use. Error out if that is the case
    if let Ok(_) = user::get_by_name(&app_state.pool, &signup_request.name).await {
        debug!("User {:?} already exists", &signup_request.name);
        return Err((StatusCode::CONFLICT,"User name already exists".to_string()));
    }

    // check if the email address is already in use. Error out if that is the case
    if let Ok(_) = user::get_by_email(&app_state.pool, &signup_request.email).await {
        debug!("Email address {:?} already exists", &signup_request.email);
        return Err((StatusCode::CONFLICT,"User email already exists".to_string()));
    }

    // Hash the password
    let password_hash = hash_password(signup_request.password)
        .map_err(|message| (StatusCode::INTERNAL_SERVER_ERROR,message))?;

    // Generate bearer token
    let token = create_jwt(&signup_request.name, false, &app_state.config.jwt_secret, app_state.config.jwt_expiration)
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR,"Error creating token".to_string()))?;
    
    // Generate validation code
    let validation_code = ValidationCode::new();
    debug!("Validation Code: {:?} for signup name {:?}", validation_code, &signup_request.name);
    
    // Try to create the user in the database
    if let Err(e) = user::create(&app_state.pool, &signup_request.name, &signup_request.email, &password_hash, &validation_code, &false).await {
        error!("Error trying to create a user during signup {:?}", e);
        return Err((StatusCode::INTERNAL_SERVER_ERROR, "Internal error creating user, please report".to_string()));
    }

    // Send validation mail
    if let Err(e) = utils::send_email(&app_state.config, &signup_request.name, &signup_request.email, &validation_code).await {
        error!("Error returned from send_mail {:?}", e);
        return Err((StatusCode::INTERNAL_SERVER_ERROR, "Internal error sending validation mail, please report".to_string()));
    }
    
    info!("User {:?} created, validation mail sent to {:?}", &signup_request.name, &signup_request.email);

    Ok((StatusCode::CREATED, "User created, validation mail sent".to_string()))

}

/// Validate route
async fn validate(State(app_state): State<AppState>,
                  Json(signup_request): Json<SignupRequest>)
               -> Result<(StatusCode, String), (StatusCode, String)> {
    // TODO: Implement validation logic
    Ok((StatusCode::ACCEPTED, "Validation successful".to_string()))
}

/// Login route
async fn login() -> Result<(StatusCode, String), (StatusCode, String)> {
    // TODO: Implement login logic
    // TODO: invalidate the account after 3 wrong logins
    // TODO: Invalidate the account when password is expired
    Ok((StatusCode::OK, "Login successful".to_string()))
}


