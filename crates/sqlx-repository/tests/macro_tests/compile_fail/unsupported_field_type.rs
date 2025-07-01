//! Test that repository derive fails with unsupported field types

use sqlx_repository::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow, Repository)]
#[repository(table = "users")]
pub struct User {
    pub id: i32,
    pub name: String,
    pub metadata: HashMap<String, String>,  // Unsupported type
}

fn main() {}