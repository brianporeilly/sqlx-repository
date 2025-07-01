//! Test that repository derive fails when no primary key is specified

use sqlx_repository::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow, Repository)]
#[repository(table = "users")]
pub struct User {
    pub name: String,  // No id field and no #[repository(primary_key)] attribute
    pub email: String,
}

fn main() {}