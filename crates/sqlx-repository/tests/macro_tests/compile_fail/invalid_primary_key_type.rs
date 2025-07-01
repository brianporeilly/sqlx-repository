//! Test that repository derive fails with non-i32 primary key

use sqlx_repository::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow, Repository)]
#[repository(table = "users")]
pub struct User {
    pub id: String,  // Primary key must be i32 for now
    pub name: String,
    pub email: String,
}

fn main() {}