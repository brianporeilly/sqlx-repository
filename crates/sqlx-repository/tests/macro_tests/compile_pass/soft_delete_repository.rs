//! Test soft delete repository derive macro compilation

use sqlx_repository::prelude::*;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow, Repository)]
#[repository(table = "users")]
#[repository(soft_delete)]
#[repository(searchable_fields(name, email))]
#[repository(filterable_fields(status, department))]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub status: String,
    pub department: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

fn main() {
    // This should compile successfully
    // Test that the repository methods are available
    let table_name = UserRepository::table_name();
    assert_eq!(table_name, "users");
    assert!(UserRepository::soft_delete_enabled());
    assert_eq!(UserRepository::searchable_fields(), &["name", "email"]);
    assert_eq!(UserRepository::filterable_fields(), &["status", "department"]);
    
    // Test that Create and Update types are generated correctly
    let _create: CreateUser = CreateUser {
        name: "test".to_string(),
        email: "test@example.com".to_string(),
        status: "active".to_string(),
        department: "engineering".to_string(),
    };
    
    let _update: UpdateUser = UpdateUser {
        name: Some("updated".to_string()),
        email: None,
        status: Some("inactive".to_string()),
        department: None,
    };
}