use serde::{Deserialize, Serialize};
use validator::{validate_email, validate_length, ValidationError};
use std::fmt;
use crate::models::Id;
use sqlx::{Type, Sqlite};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, sqlx::Type)]
pub struct User {
    pub user_id: Id,
    pub name: Name,
    pub email: Email,
    pub password_hash: String,
    pub is_admin: bool,
    pub is_valid: bool,
    pub is_active: bool
}

/// Create the Password type
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(transparent)]
pub struct Password (String);

/// Implement the Password type
impl Password {
    pub fn new (password: &str) -> Result<Self, Vec<&'static str>> {
        let mut errors = Vec::new();
        
        if !validate_length(password, Some(8), Some(50), None) {
            errors.push("Password must be between 8 and 50 characters");
        }
        
        // Add custom password rules
        if !password.chars().any(|c| c.is_uppercase()) {
            errors.push("Password must contain uppercase letter");
        }
        if !password.chars().any(|c| c.is_lowercase()) {
            errors.push("Password must contain lowercase letter");
        }
        if !password.chars().any(|c| c.is_numeric()) {
            errors.push("Password must contain number");
        }
        if !password.chars().any(|c| !c.is_alphanumeric()) {
            errors.push("Password must contain special character");
        }

        if errors.is_empty() {
            Ok(Password(password.to_string()))
        } else {
            Err(errors)
        }
    }
    pub fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes() // Assuming Password wraps a String or similar
    }
}

impl From<String> for Password {
    fn from(password: String) -> Self {
        Password(password)
    }
}

/*
impl TryFrom<String> for Password {
    type Error = Vec<&'static str>;
    
    fn try_from(password: String) -> Result<Self, Vec<&'static str>> {
        Password::new(&password)
    }
}
*/

/// Create the Name type
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(transparent)]
pub struct Name(String);

/// Implement the Password type
impl Name {
    pub fn new(name: &str) -> Result<Self, Vec<&'static str>> {
        let mut errors = Vec::new();
        
        // Nme must be between 3 and 50 characters
        if !validate_length(name, Some(8), Some(50), None) {
            errors.push("Name must be between 3 and 50 characters");
        }
        
        // Name must start with a letter
        if !name.chars().next().map_or(false, |c| c.is_alphabetic()) {
            errors.push("Name must start with a letter");
        }

        if errors.is_empty() {
            Ok(Name(name.to_string()))
        } else {
            Err(errors)
        }
    }
}

impl From<String> for Name {
    fn from(value: String) -> Self {
        Name(value)
    }
}

/// Create the Email type
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(transparent)]
pub struct Email(String);

/// Implement the Email type
impl Email {
    pub fn new(email: &str) -> Result<Self, &'static str> {
        if validate_email(email) {
            Ok(Email(email.to_string()))
        } else {
            Err("Invalid email format")
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
    
}

impl From<Email> for String {
    fn from(email: Email) -> Self {
        email.0
    }
}

impl From<String> for Email {
    fn from(email: String) -> Self {
        Email(email)
    }
}
/*
impl TryFrom<String> for Email {
    type Error = &'static str;
    
    fn try_from(email: String) -> Result<Self, Self::Error> {
        Email::new(&email)
    }
}
*/

impl fmt::Display for Email {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
