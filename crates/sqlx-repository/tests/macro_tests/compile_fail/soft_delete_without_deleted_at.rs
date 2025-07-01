//! Test that soft delete requires deleted_at field

use sqlx_repository::prelude::*;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow, Repository)]
#[repository(table = "users")]
#[repository(soft_delete)]  // This should fail without deleted_at field
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    // Missing: pub deleted_at: Option<DateTime<Utc>>,
}

fn main() {}