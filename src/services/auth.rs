/*
use anyhow::Result;
use crate::models::user::User;
use crate::utils::jwt::create_jwt;

/// Authenticate a user and generate a JWT token
pub fn authenticate_user(user: User, password: &str) -> Result<String> {
    // TODO: Validate password and generate JWT
    let token = create_jwt(&user.name, user.is_admin, state., 3600)?;
    Ok(token)
}
*/