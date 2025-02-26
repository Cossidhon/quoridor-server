use serde::{Deserialize, Serialize};
use sqlx::Type;

pub mod user;

pub use user::User;


/// Create the Password type
#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[sqlx(transparent)]
pub struct Id(i64);

impl From<i64> for Id {
    fn from(value: i64) -> Self {
        Id(value)
    }
}