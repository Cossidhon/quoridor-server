pub mod user;

pub use user::{db_create_user, db_get_user_by_email, db_get_all_users, db_get_user_by_id, db_update_user, db_delete_user};

use sqlx::{SqlitePool, migrate::MigrateDatabase};
use anyhow::Result;

pub async fn connect(database_url: &str) -> Result<SqlitePool> {
    // Create the database if it doesn't exist
    if !sqlx::Sqlite::database_exists(database_url).await? {
        sqlx::Sqlite::create_database(database_url).await?;
    }

    // Connect to the database
    let pool = SqlitePool::connect(database_url).await?;

    // Run migrations
    sqlx::migrate!("./migrations").run(&pool).await?;

    Ok(pool)
}