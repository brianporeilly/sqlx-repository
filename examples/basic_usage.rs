//! Basic usage example showing CRUD operations
//!
//! This example demonstrates the fundamental operations of the sqlx-repository
//! crate including creating, reading, updating, and deleting entities.

use sqlx_repository::prelude::*;
use sqlx::PgPool;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Example User entity with soft delete support
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow, RepositoryDerive)]
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

/// Custom repository methods for User
impl UserRepository {
    /// Find a user by email address
    pub async fn find_by_email(&self, email: &str) -> RepositoryResult<Option<User>> {
        let query = if Self::soft_delete_enabled() {
            "SELECT * FROM users WHERE email = $1 AND deleted_at IS NULL"
        } else {
            "SELECT * FROM users WHERE email = $1"
        };

        sqlx::query_as(query)
            .bind(email)
            .fetch_optional(self.pool())
            .await
            .map_err(RepositoryError::from)
    }

    /// Find all active users
    pub async fn find_active_users(&self) -> RepositoryResult<Vec<User>> {
        let query = if Self::soft_delete_enabled() {
            "SELECT * FROM users WHERE status = 'active' AND deleted_at IS NULL ORDER BY created_at DESC"
        } else {
            "SELECT * FROM users WHERE status = 'active' ORDER BY created_at DESC"
        };

        sqlx::query_as(query)
            .fetch_all(self.pool())
            .await
            .map_err(RepositoryError::from)
    }

    /// Count users by department
    pub async fn count_by_department(&self, department: &str) -> RepositoryResult<i64> {
        let query = if Self::soft_delete_enabled() {
            "SELECT COUNT(*) FROM users WHERE department = $1 AND deleted_at IS NULL"
        } else {
            "SELECT COUNT(*) FROM users WHERE department = $1"
        };

        sqlx::query_scalar(query)
            .bind(department)
            .fetch_one(self.pool())
            .await
            .map_err(RepositoryError::from)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Setup environment and database connection
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://test_user:test_pass@localhost:5432/test_db".to_string());
    
    println!("Connecting to database: {}", database_url);
    let pool = PgPool::connect(&database_url).await?;
    
    // Create repository
    let user_repo = UserRepository::new(pool);
    
    println!("\n=== Basic CRUD Operations ===");
    
    // Create a new user
    let create_user = CreateUser {
        name: "Alice Johnson".to_string(),
        email: "alice@example.com".to_string(),
        status: "active".to_string(),
        department: "engineering".to_string(),
    };
    
    let user = user_repo.create(create_user).await?;
    println!("✓ Created user: {} (ID: {})", user.name, user.id);
    
    // Find user by ID
    let found_user = user_repo.find_by_id(user.id).await?;
    match found_user {
        Some(u) => println!("✓ Found user by ID: {}", u.name),
        None => println!("✗ User not found"),
    }
    
    // Find user by email (custom method)
    let found_by_email = user_repo.find_by_email(&user.email).await?;
    match found_by_email {
        Some(u) => println!("✓ Found user by email: {}", u.name),
        None => println!("✗ User not found by email"),
    }
    
    // Update user
    let update_user = UpdateUser {
        name: Some("Alice Smith".to_string()),
        email: None, // Don't update email
        status: Some("senior".to_string()),
        department: None, // Don't update department
    };
    
    let updated_user = user_repo.update(user.id, update_user).await?;
    match updated_user {
        Some(u) => println!("✓ Updated user: {} (status: {})", u.name, u.status),
        None => println!("✗ User not found for update"),
    }
    
    println!("\n=== Search and Filtering ===");
    
    // Search users by name
    let search_params = SearchParams {
        query: Some("Alice".to_string()),
        ..Default::default()
    };
    
    let search_results = user_repo.search(search_params).await?;
    println!("✓ Found {} users matching 'Alice'", search_results.items.len());
    for user in &search_results.items {
        println!("  - {} ({})", user.name, user.email);
    }
    
    // Filter users by department
    let mut filters = std::collections::HashMap::new();
    filters.insert("department".to_string(), "engineering".to_string());
    
    let filter_params = SearchParams {
        filters,
        per_page: 5,
        sort_by: Some("name".to_string()),
        sort_order: SortOrder::Asc,
        ..Default::default()
    };
    
    let filtered_results = user_repo.search(filter_params).await?;
    println!("✓ Found {} users in engineering department", filtered_results.items.len());
    
    // Count users by department (custom method)
    let eng_count = user_repo.count_by_department("engineering").await?;
    println!("✓ Total engineering users: {}", eng_count);
    
    println!("\n=== Pagination ===");
    
    // Paginated results
    let page_params = SearchParams {
        page: 0,
        per_page: 2,
        ..Default::default()
    };
    
    let page_results = user_repo.search(page_params).await?;
    println!("✓ Page 1: {} of {} total users", page_results.items.len(), page_results.total_count);
    println!("  Total pages: {}", page_results.total_pages);
    println!("  Has next page: {}", page_results.has_next_page());
    
    println!("\n=== Soft Delete Operations ===");
    
    // Soft delete user
    let deleted = user_repo.delete(user.id).await?;
    if deleted {
        println!("✓ Soft deleted user: {}", user.name);
    }
    
    // Try to find deleted user (should not be found)
    let found_deleted = user_repo.find_by_id(user.id).await?;
    match found_deleted {
        Some(_) => println!("✗ Deleted user still found (should not happen)"),
        None => println!("✓ Deleted user correctly not found in normal queries"),
    }
    
    // Search for deleted records
    let deleted_params = SearchParams {
        scope: RecordScope::Deleted,
        ..Default::default()
    };
    
    let deleted_results = user_repo.search(deleted_params).await?;
    println!("✓ Found {} deleted users", deleted_results.items.len());
    
    // Restore user
    let restored_user = user_repo.restore(user.id).await?;
    match restored_user {
        Some(u) => println!("✓ Restored user: {}", u.name),
        None => println!("✗ Failed to restore user"),
    }
    
    // Verify user is restored
    let found_restored = user_repo.find_by_id(user.id).await?;
    match found_restored {
        Some(u) => println!("✓ Restored user found: {}", u.name),
        None => println!("✗ Restored user not found"),
    }
    
    // Hard delete (permanent)
    let hard_deleted = user_repo.hard_delete(user.id).await?;
    if hard_deleted {
        println!("✓ Permanently deleted user");
    }
    
    println!("\n=== Summary ===");
    
    // Get final count
    let final_count = user_repo.count(SearchParams::default()).await?;
    println!("✓ Final user count: {}", final_count);
    
    // List all active users
    let active_users = user_repo.find_active_users().await?;
    println!("✓ Active users:");
    for user in active_users {
        println!("  - {} ({}) - {}", user.name, user.email, user.department);
    }
    
    println!("\n🎉 Example completed successfully!");
    
    Ok(())
}