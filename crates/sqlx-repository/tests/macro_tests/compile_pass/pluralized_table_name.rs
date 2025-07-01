//! Test automatic table name pluralization

use sqlx_repository::prelude::*;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

// Test basic pluralization (user -> users)
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow, Repository)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// Test 'y' to 'ies' pluralization (category -> categories)
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow, Repository)]
pub struct Category {
    pub id: i32,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// Test 's' pluralization (address -> addresses)
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow, Repository)]
pub struct Address {
    pub id: i32,
    pub street: String,
    pub city: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

fn main() {
    // Test that pluralization works correctly
    assert_eq!(UserRepository::table_name(), "users");
    assert_eq!(CategoryRepository::table_name(), "categories");
    assert_eq!(AddressRepository::table_name(), "addresses");
    
    // Test that Create and Update types are generated
    let _user_create: CreateUser = CreateUser {
        name: "test".to_string(),
        email: "test@example.com".to_string(),
    };
    
    let _category_create: CreateCategory = CreateCategory {
        name: "test category".to_string(),
    };
    
    let _address_create: CreateAddress = CreateAddress {
        street: "123 Main St".to_string(),
        city: "Test City".to_string(),
    };
}