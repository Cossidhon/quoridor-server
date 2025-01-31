use sqlx::SqlitePool;
use anyhow::Result;
use crate::models::user::User;

/// Get all users from the database
pub async fn db_get_all_users(pool: &SqlitePool) -> Result<Vec<User>> {
    let users = sqlx::query_as!(
        User,
        r#"
        SELECT userid, role, password, email
        FROM user
        "#
    )
    .fetch_all(pool)
    .await?;

    Ok(users)
}

/// Get a user by userid
pub async fn db_get_user_by_id(pool: &SqlitePool, userid: &str) -> Result<Option<User>> {
    let user = sqlx::query_as!(
        User,
        r#"
        SELECT userid, role, password, email
        FROM user
        WHERE userid = ?
        "#,
        userid
    )
    .fetch_optional(pool)
    .await?;

    Ok(user)
}

/// Update a user's role and email
pub async fn db_update_user(pool: &SqlitePool, userid: &str, role: &str, email: &str) -> Result<()> {
    sqlx::query!(
        r#"
        UPDATE user
        SET role = ?, email = ?
        WHERE userid = ?
        "#,
        role,
        email,
        userid
    )
    .execute(pool)
    .await?;

    Ok(())
}

/// Create a new user in the database
pub async fn db_create_user(
    pool: &SqlitePool,
    userid: &str,
    role: &str,
    password: &str,
    email: &str,
) -> Result<()> {
    sqlx::query!(
        r#"
        INSERT INTO user (userid, role, password, email)
        VALUES (?, ?, ?, ?)
        "#,
        userid,
        role,
        password,
        email
    )
    .execute(pool)
    .await?;

    Ok(())
}

/// Delete a user by userid
pub async fn db_delete_user(pool: &SqlitePool, userid: &str) -> Result<()> {
    sqlx::query!(
        r#"
        DELETE FROM user
        WHERE userid = ?
        "#,
        userid
    )
    .execute(pool)
    .await?;

    Ok(())
}

/// Get a user by email
pub async fn db_get_user_by_email(pool: &SqlitePool, email: &str) -> Result<Option<User>> {
    let user = sqlx::query_as!(
        User,
        r#"
        SELECT userid, role, password, email
        FROM user
        WHERE email = ?
        "#,
        email
    )
    .fetch_optional(pool)
    .await?;

    Ok(user)
}