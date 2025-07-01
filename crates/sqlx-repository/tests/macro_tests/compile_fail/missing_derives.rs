//! Test that repository derive fails when required derives are missing

use sqlx_repository::prelude::*;

// Missing Debug, Clone, Serialize, Deserialize, sqlx::FromRow
#[derive(Repository)]
#[repository(table = "users")]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
}

fn main() {}