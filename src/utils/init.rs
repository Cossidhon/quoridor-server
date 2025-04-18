use std::env;
// use anyhow::{anyhow, Result};
use tracing::{info, error, debug};
use crate::{db::user, AppState};
use axum::http::StatusCode;
use crate::models::user::{Name, Email, Password};
use crate::utils::hash_password;
use crate::models::user::ValidationCode;


// this function will check if the user table is empty. If it is, it will create the admin user
// If the env var INIT is set to true, it will first try to delete the admin account
pub async fn init_check(app_state: &AppState) -> Result<(), (StatusCode, String)> {

    // init y/n
    let init = env::var("INIT")
        .unwrap_or_else(|_| "false".to_string()) // Default to false if not provided
        .parse::<bool>()
        .map_err(|_| (StatusCode::BAD_GATEWAY, "INIT must be a boolean (true or false)".to_string()))?;
    
    // admin user
    let admin_user_name= Name::from(env::var("ADMIN_USER_NAME")
        .unwrap_or_else(|_| "admin".to_string()));

    // admin email
    let admin_email_address = Email::from(env::var("ADMIN_EMAIL_ADDRESS")
        .unwrap_or_else(|_| admin_user_name.to_string() + "@localhost"));

    // admin password
    let admin_password = Password::from(env::var("ADMIN_PASSWORD")
        .unwrap_or_else(|_| "Qu0r1d0R!".to_string()));

    if init {

        info!("INIT mode!");

        // Try to delete the admin user
        if let Err(_) = user::delete(&app_state.pool, &admin_user_name).await {
            info!("Error trying to delete existing admin user during INIT, ignoring");
        };
    };

    // Check if the user table is empty
    let result = user::get_user_count(&app_state.pool).await;
    let user_count = match result {
        Ok(Some(count)) => count,
        Ok(None) => 0,
        Err(_) => return Err((StatusCode::INTERNAL_SERVER_ERROR, "Error counting users".to_string())),
    };
    
    // Create admin user if user table is empty
    if user_count == 0 {

        // Try to create the user in the database
        let validation_code = ValidationCode::from(0);
        let id_admin = false;

        // Hash the password
        let password_hash = hash_password(admin_password)
            .map_err(|message| (StatusCode::INTERNAL_SERVER_ERROR,message))?;

        if let Err(e) = user::create(&app_state.pool,
                                     &admin_user_name,
                                     &admin_email_address,
                                     &password_hash,
                                     &validation_code,
                                     &id_admin).await {
            error!("Error creating admin user {:?}", e);
            return Err((StatusCode::INTERNAL_SERVER_ERROR, "Internal error creating admin user".to_string()));
        }
    }

    Ok(())
}
