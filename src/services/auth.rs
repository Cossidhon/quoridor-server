use anyhow::Result;
use crate::models::user::User;
use crate::utils::jwt::create_jwt;

/// Authenticate a user and generate a JWT token
pub fn authenticate_user(user: &User, password: &str) -> Result<String> {
    // TODO: Validate password and generate JWT
    let token = create_jwt(&user.userid, &user.role, "your_jwt_secret", 3600)?;
    Ok(token)
}