use dotenv::dotenv;
use std::env;
use anyhow::{anyhow, Context, Result};
use validator::validate_email; // For email validation
use regex::Regex; // For FQDN validation

#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub port: u16,
    pub jwt_secret: String,
    pub jwt_expiration: String,
    pub email_from_address: String,
    pub email_smtp_host: String,
    pub email_smtp_username: String,
    pub email_smtp_password: String,
}

impl Config {
    pub fn load() -> Result<Self> {
        dotenv().ok();

        // Load and validate environment variables
        let database_url = env::var("DATABASE_URL")
            .map_err(|_| anyhow!("DATABASE_URL is required and must be a valid SQLite connection string"))?;

        let port = env::var("PORT")
            .unwrap_or_else(|_| "3000".to_string()) // Default to 3000 if not provided
            .parse::<u16>()
            .map_err(|_| anyhow!("PORT must be a valid number between 1 and 65535"))?;

        let jwt_secret = env::var("JWT_SECRET")
            .map_err(|_| anyhow!("JWT_SECRET is required and must be a non-empty string"))?;

        let jwt_expiration = env::var("JWT_EXPIRATION")
            .unwrap_or_else(|_| "1h".to_string()); // Default to 1h if not provided

        let email_from_address = env::var("EMAIL_FROM_ADDRESS")
            .map_err(|_| anyhow!("EMAIL_FROM_ADDRESS is required and must be a valid email address"))?;

        // Validate email format
        if !validate_email(&email_from_address) {
            return Err(anyhow!("EMAIL_FROM_ADDRESS is not a valid email address"));
        }

        let email_smtp_host = env::var("EMAIL_SMTP_HOST")
            .map_err(|_| anyhow!("EMAIL_SMTP_HOST is required and must be a valid FQDN"))?;

        // Validate FQDN format using a custom regex
        if !is_valid_fqdn(&email_smtp_host)? {
            return Err(anyhow!("EMAIL_SMTP_HOST is not a valid FQDN"));
        }

        let email_smtp_username = env::var("EMAIL_SMTP_USERNAME")
            .map_err(|_| anyhow!("EMAIL_SMTP_USERNAME is required and must be a non-empty string"))?;

        let email_smtp_password = env::var("EMAIL_SMTP_PASSWORD")
            .map_err(|_| anyhow!("EMAIL_SMTP_PASSWORD is required and must be a non-empty string"))?;

        Ok(Config {
            database_url,
            port,
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