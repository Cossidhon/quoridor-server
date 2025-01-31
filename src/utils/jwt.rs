use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // User ID
    pub role: String,
    pub exp: usize,  // Expiration time
}

/// Create a JWT token
pub fn create_jwt(user_id: &str, role: &str, secret: &str, expiration: usize) -> Result<String> {
    let claims = Claims {
        sub: user_id.to_string(),
        role: role.to_string(),
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