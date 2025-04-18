/*
use anyhow::Result;
use crate::models::user::{User, Name, Email, Password};
use crate::db::user;
use crate::models::Id;

/// (ADMIN ONLY) Get all users
pub async fn get_all(pool: &sqlx::SqlitePool) -> Result<Vec<User>> {
    // TODO: check if user is admin
    let users = user::get_all(pool).await?;
    Ok(users)
}

/// (ADMIN ONLY) Get a user
pub async fn get(pool: &sqlx::SqlitePool, user_id: Id) -> Result<Option<User>> {
    // TODO: check if user is admin
    let user = user::get(pool, user_id).await?;
    Ok(user)
}

/// Delete a user
pub async fn delete(pool: &sqlx::SqlitePool, user_id: Id) -> Result<()> {
    // TODO: check if user is self or admin
    user::delete(pool, user_id).await?;
    Ok(())
}

/// Change a user's password
pub async fn change_password(pool: &sqlx::SqlitePool, email: Email, password: Password) -> Result<()> {
    // TODO: check if user is self of admin
    let password_hash = "hash".to_string();
    user::change_password(pool, email, password_hash).await?;
    Ok(())
}

/// Change a user's email
pub async fn change_email(pool: &sqlx::SqlitePool, email: Email, new_email: Email) -> Result<()> {
    // TODO: check if user is self or admin
    user::change_email(pool, email, new_email).await?;
    // TODO: change email adress in other tables as email is used as the key
    Ok(())
}

/// Create a new user
pub async fn create(_pool: &sqlx::SqlitePool, name: Name, email: Email, password: Password) -> Result<()> {
    // TODO: validate input
    // TODO: validate if user already exists
    // TODO: hash password
    //user::create(pool, name, email, password_hash).await?;
    Ok(())
}
*/