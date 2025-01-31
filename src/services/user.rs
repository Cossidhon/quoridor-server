use anyhow::Result;
use crate::models::user::User;
use crate::db::user::{db_get_all_users, db_get_user_by_id, db_update_user, db_delete_user};

/// Get all users
pub async fn get_all_users(pool: &sqlx::SqlitePool) -> Result<Vec<User>> {
    let users = db_get_all_users(pool).await?;
    Ok(users)
}

/// Get a user by ID
pub async fn get_user_by_id(pool: &sqlx::SqlitePool, userid: &str) -> Result<Option<User>> {
    let user = db_get_user_by_id(pool, userid).await?;
    Ok(user)
}

/// Update a user's information
pub async fn update_user_info(
    pool: &sqlx::SqlitePool,
    userid: &str,
    role: &str,
    email: &str,
) -> Result<()> {
    db_update_user(pool, userid, role, email).await?;
    Ok(())
}

/// Delete a user by ID
pub async fn delete_user_by_id(pool: &sqlx::SqlitePool, userid: &str) -> Result<()> {
    db_delete_user(pool, userid).await?;
    Ok(())
}