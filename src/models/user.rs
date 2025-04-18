use jsonwebtoken::Validation;
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
    pub validation_code: i64, // TODO: Make a type ValidationCode for it
    pub is_admin: bool,
    pub is_valid: bool,
    pub is_active: bool
}

/// Create the Password newtype
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

        // All Password rules comply, return the password
        if errors.is_empty() {
            Ok(Self(password.to_string()))
        } else {
            Err(errors)
        }
    }

    pub fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes() // Assuming Password wraps a String or similar
    }
}

// Converting a String to a password type
impl From<String> for Password {
    fn from(password: String) -> Self {
        Password(password)
    }
}

/// Create the Name newtype
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(transparent)]
pub struct Name(String);

/// Implement the Name type
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

        // All Name rules comply, return the Name
        if errors.is_empty() {
            Ok(Self(name.to_string()))
        } else {
            Err(errors)
        }
    }
}

// Convert String to a Name type
impl From<String> for Name {
    fn from(value: String) -> Self {
        Self(value)
    }
}

// Implement the Display trait for Name
impl fmt::Display for Name {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}


/// Create the Email newtype
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(transparent)]
pub struct Email(String);

/// Implement the Email type
impl Email {
    pub fn new(email: &str) -> Result<Self, &'static str> {
        if validate_email(email) {
            Ok(Self(email.to_string()))
        } else {
            Err("Invalid email format")
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
    
}
/*
// Convert from Email to String
impl From<Email> for String {
    fn from(email: Email) -> Self {
        email.0
    }
}
*/

// Convert String to Email
impl From<String> for Email {
    fn from(email: String) -> Self {
        Self(email)
    }
}

// Implement Display trait for Email
impl fmt::Display for Email {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// Create the ValidationCode newtype
#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(transparent)]
pub struct ValidationCode(i64);

impl ValidationCode {
    pub fn new() -> Self {
        Self(rand::random_range(10000..=99999))
    }
}

// Convert from ValidationCode to i64
impl From<ValidationCode> for i64 {
    fn from(validation_code: ValidationCode) -> Self {
        validation_code.0
    }
}

// Convert i64 to ValidationCode
impl From<i64> for ValidationCode {
    fn from(validation_code: i64) -> Self {
        Self(validation_code)
    }
}

// Implement Display trait for ValidationCode
impl fmt::Display for ValidationCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
