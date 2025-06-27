//! Test basic repository derive macro compilation

use sqlx_repository::prelude::*;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow, Repository)]
#[repository(table = "users")]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

fn main() {
    // This should compile successfully
    // Test that the macro generates the expected types and methods
    let table_name = UserRepository::table_name();
    assert_eq!(table_name, "users");
    
    // Test that Create and Update types are generated
    let _create: CreateUser = CreateUser {
        name: "test".to_string(),
        email: "test@example.com".to_string(),
    };
    
    let _update: UpdateUser = UpdateUser {
        name: Some("updated".to_string()),
        email: None,
    };
}