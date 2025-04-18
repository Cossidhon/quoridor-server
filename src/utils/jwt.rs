use std::time::Duration;
use std::env;
use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use serde::{Deserialize, Serialize};
use anyhow::Result;
use crate::models::user::Name;
use tracing::debug;
use chrono::{TimeDelta, Utc};

// The claims struct used for creating a Bearer token
#[derive(Deserialize, Serialize, Debug)]
struct Claims {
    sub: String,
    is_admin: bool,
    iat: usize,
    exp: usize,
}



/// Create a JWT token
pub fn create_jwt(user_name: &Name, is_admin: bool, secret: &str, expiration: i64) -> Result<String> {

    // Define the registered <Expiration Time> claim (exp) which is the current timestmap plus the defined offset
    let exp = Utc::now()
        .checked_add_signed(TimeDelta::minutes(expiration))
        .expect("invalid timestamp")
        .timestamp();

    // Bui;d the Claims struct
    let claim = Claims {
        sub: user_name.to_string(),                 // user name
        is_admin,                                   // is user admin?
        iat: Utc::now().timestamp() as usize,       // valid from
        exp: exp as usize,                          // valid until
    };

    // Encode bearer token
    let token = encode(&Header::default(), &claim, &EncodingKey::from_secret(secret.as_ref()))?;
    debug!("JWT bearer token generated: {}", &token);
    Ok(token)
}

/// Validate a JWT token
pub fn validate_jwt(token: &str, secret: &str) -> Result<Claims> {
    let token_data = decode::<Claims>(token, &DecodingKey::from_secret(secret.as_ref()), &Validation::default())?;
    Ok(token_data.claims)
}