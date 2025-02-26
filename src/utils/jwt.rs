use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use serde::{Deserialize, Serialize};
use anyhow::Result;
use crate::models::user::Email;


#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub email: Email, // User Email
    pub is_admin: bool, // Admin flag
    pub exp: usize,  // Expiration time
}

/// Create a JWT token
pub fn create_jwt(email: &Email, is_admin: bool, secret: &str, expiration: usize) -> Result<String> {
    let claims = Claims {
        email,
        is_admin,
        exp: expiration,
    };
    let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_ref()))?;
    Ok(token)
}

/// Validate a JWT token
pub fn validate_jwt(token: &str, secret: &str) -> Result<Claims> {
    let token_data = decode::<Claims>(token, &DecodingKey::from_secret(secret.as_ref()), &Validation::default())?;
    Ok(token_data.claims)
}