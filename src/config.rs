use dotenv::dotenv;
use std::env;
use anyhow::{anyhow, Context, Result};
use validator::validate_email; // For email validation
use regex::Regex; // For FQDN validation

#[derive(Debug, Clone)]
pub struct Config {
    pub jwt_secret: String,
    pub jwt_expiration: i64,
    pub email_from_address: String,
    pub email_smtp_host: String,
    pub email_smtp_username: String,
    pub email_smtp_password: String,
}

impl Config {

    // Load and validate environment variables
    pub fn load() -> Result<Self> {
        dotenv().ok();

        // jwt secret. Default is "Quoridor Server"
        let jwt_secret = env::var("JWT_SECRET")
            .unwrap_or_else(|_| "Quoridor Server".to_string());

        // token expiration in minutes. Default is 300 minutes if not provided
        let jwt_expiration = env::var("JWT_DURATION")
            .unwrap_or_else(|_| "300".to_string()).parse::<i64>().expect("JWT_DURATION is not numeric");

        // email return address
        let email_from_address = env::var("EMAIL_FROM_ADDRESS")
            .map_err(|_| anyhow!("EMAIL_FROM_ADDRESS is required and must be a valid email address"))?;

        // Validate email format
        if !validate_email(&email_from_address) {
            return Err(anyhow!("EMAIL_FROM_ADDRESS is not a valid email address"));
        }

        // email smtp host
        let email_smtp_host = env::var("EMAIL_SMTP_HOST")
            .map_err(|_| anyhow!("EMAIL_SMTP_HOST is required and must be a valid FQDN"))?;

        // Validate FQDN format using a custom regex
        if !is_valid_fqdn(&email_smtp_host)? {
            return Err(anyhow!("EMAIL_SMTP_HOST is not a valid FQDN"));
        }

        // smtp server username
        let email_smtp_username = env::var("EMAIL_SMTP_USERNAME")
            .map_err(|_| anyhow!("EMAIL_SMTP_USERNAME is required and must be a non-empty string"))?;

        // smtp server password
        let email_smtp_password = env::var("EMAIL_SMTP_PASSWORD")
            .map_err(|_| anyhow!("EMAIL_SMTP_PASSWORD is required and must be a non-empty string"))?;

        Ok(Config {
            jwt_secret,
            jwt_expiration,
            email_from_address,
            email_smtp_host,
            email_smtp_username,
            email_smtp_password,
        })
    }
}

/// Validates if a string is a valid FQDN (Fully Qualified Domain Name)
fn is_valid_fqdn(fqdn: &str) -> Result<bool> {
    // Compile the regex and handle errors gracefully
    let re = Regex::new(r"^([a-zA-Z0-9]+(-[a-zA-Z0-9]+)*\.)+[a-zA-Z]{2,}$")
        .context("Failed to compile FQDN validation regex")?;

    Ok(re.is_match(fqdn))
}