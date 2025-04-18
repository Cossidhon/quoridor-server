use argon2::{Argon2, PasswordHasher, PasswordVerifier, password_hash::{SaltString, rand_core::OsRng}};
use crate::models::user::Password;
use tracing::{debug,error};

pub fn hash_password(password: Password) -> Result<String, String> {
    
    // Generate a random salt
    let salt = SaltString::generate(&mut OsRng);

    // Create an Argon2 instance
    let argon2 = Argon2::default();

    // Hash the password
    let hash = match argon2.hash_password(password.as_bytes(), &salt){
        Ok(hash) => hash,
        Err(_) => {
            error!("Error hashing password");
            return Err("Error hashing password".to_string());
        }
    };

    debug!("Password Hash: {}", hash);

    // Return the hashed password
    Ok(hash.to_string())
}

/*
pub fn verify_password(hashed: &str, password: Password) -> bool {
    let argon2 = Argon2::default();
    
    // Attempt to verify the password
    match argon2.verify_password(password.as_bytes(), &hashed.into()) {
        Ok(_) => true, // Password is correct
        Err(_) => false, // Password is incorrect
    }
}
*/