use sqlx::SqlitePool;
use anyhow::Result;
use crate::models::user::{Email, Name, Password, User, ValidationCode};
use crate::models::Id;

/// Get all users from the database
pub async fn get_all(pool: &SqlitePool) -> Result<Vec<User>> {
    let users = sqlx::query_as!(
        User,
        r#"
        SELECT user_id, name, email, password_hash, validation_code, is_admin, is_valid, is_active
        FROM user
        "#
    )
    .fetch_all(pool)
    .await?;

    Ok(users)
}

/// Get a user
pub async fn get(pool: &SqlitePool, user_id: Id) -> Result<Option<User>> {
    let user = sqlx::query_as!(
        User,
        r#"
        SELECT user_id, name, email, password_hash, validation_code, is_admin, is_valid, is_active
        FROM user
        WHERE user_id = ?
        "#,
        user_id
    )
    .fetch_one(pool)
    .await?;

    Ok(Some(user))
}

/// Get a user by name
pub async fn get_by_name(pool: &SqlitePool, name: &Name) -> Result<Option<User>> {
    let user = sqlx::query_as!(
        User,
        r#"
        SELECT user_id, name, email, password_hash, validation_code, is_admin, is_valid, is_active
        FROM user
        WHERE name = ?
        "#,
        name
    )
    .fetch_one(pool)
    .await?;

    Ok(Some(user))
}

/// Get a user by email address
pub async fn get_by_email(pool: &SqlitePool, email: &Email) -> Result<Option<User>> {
    let user = sqlx::query_as!(
        User,
        r#"
        SELECT user_id, name, email, password_hash, validation_code, is_admin, is_valid, is_active
        FROM user
        WHERE email = ?
        "#,
        email
    )
    .fetch_one(pool)
    .await?;

    Ok(Some(user))
}

/// Update all user's fields, except for email and password
pub async fn update(pool: &SqlitePool, user_id: Id, name: Name, email: Email, is_admin: &bool, is_valid: &bool, is_active: &bool) -> Result<()> {
    sqlx::query!(
        r#"
        UPDATE user
        SET name = ?, email = ?, is_admin = ?, is_valid = ?, is_active = ?
        WHERE user_id = ?
        "#,
        name,
        email,
        is_admin,
        is_valid,
        is_active,
        user_id
    )
    .execute(pool)
    .await?;

    Ok(())
}

/// Create a new user in the database, all flags (except id_admin) are set to default values
pub async fn create(pool: &SqlitePool, name: &Name, email: &Email, password_hash: &str, validation_code: &ValidationCode, is_admin: &bool) -> Result<()> {

    sqlx::query!(
        r#"
        INSERT INTO user (name, email, password_hash, validation_code, is_admin)
        VALUES (?, ?, ?, ?, ?)
        "#,
        name,
        email,
        password_hash,
        validation_code,
        is_admin
    )
    .execute(pool)
    .await?;

    Ok(())
}

/// Delete a user by name
pub async fn delete(pool: &SqlitePool, user_name: &Name) -> Result<()> {
    sqlx::query!(
        r#"
        DELETE FROM user
        WHERE name = ?
        "#,
        user_name
    )
    .execute(pool)
    .await?;

    Ok(())
}

/// Change password of a user
pub async fn change_password(pool: &SqlitePool, email: Email, password_hash: String) -> Result<()> {
    sqlx::query!(
        r#"
        UPDATE user
        SET password_hash = ?
        WHERE email = ?
        "#,
        password_hash,
        email
    )
    .execute(pool)
    .await?;

    Ok(())
}

/// Change email of a user
pub async fn change_email(pool: &SqlitePool, email: Email, new_email: Email) -> Result<()> {
    sqlx::query!(
        r#"
        UPDATE user
        SET email = ?
        WHERE email = ?
        "#,
        email,
        new_email
    )
    .execute(pool)
    .await?;

    Ok(())
}

/// Get count of user table
pub async fn get_user_count(pool: &SqlitePool) -> Result<Option<i64>> {

    let count: i64 = sqlx::query_scalar("SELECT COUNT(ROWID) FROM user")
        .fetch_one(pool)
        .await?;

    Ok(Some(count))
}