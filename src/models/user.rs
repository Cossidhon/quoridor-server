use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub userid: String,
    pub role: String,
    pub password: String,
    pub email: String,
}