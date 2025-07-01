//! Basic usage example for sqlx-repository
//! 
//! This example demonstrates the fundamental features:
//! - Defining a repository struct
//! - Basic CRUD operations
//! - Search and pagination

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use sqlx_repository::{Repository, SearchParams, RepositoryError};

// Define your entity struct with the Repository derive
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow, Repository)]
#[repository(table = "users")]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub age: Option<i32>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Setup database connection
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:password@localhost/sqlx_repository_dev".to_string());
    
    let pool = PgPool::connect(&database_url).await?;
    
    // Create repository instance
    let user_repo = UserRepository::new(pool);
    
    // Example: Create a new user
    let new_user = CreateUser {
        name: "Alice Smith".to_string(),
        email: "alice@example.com".to_string(),
        age: Some(30),
    };
    
    match user_repo.create(new_user).await {
        Ok(user) => {
            println!("✅ Created user: {} (ID: {})", user.name, user.id);
            
            // Example: Find user by ID
            if let Ok(Some(found_user)) = user_repo.find_by_id(user.id).await {
                println!("🔍 Found user: {}", found_user.name);
            }
            
            // Example: Update user
            let update_data = UpdateUser {
                name: Some("Alice Johnson".to_string()),
                age: Some(Some(31)),
                ..Default::default()
            };
            
            if let Ok(Some(updated_user)) = user_repo.update(user.id, update_data).await {
                println!("📝 Updated user: {}", updated_user.name);
            }
            
            // Example: Search with pagination
            let search_params = SearchParams {
                page: 0,
                per_page: 10,
                ..Default::default()
            };
            
            if let Ok(search_result) = user_repo.search(search_params).await {
                println!("📋 Found {} users (page {} of {})", 
                    search_result.items.len(),
                    search_result.page,
                    search_result.total_pages
                );
            }
            
            // Example: Delete user
            if let Ok(_) = user_repo.delete(user.id).await {
                println!("🗑️  Deleted user");
            }
        }
        Err(RepositoryError::Validation(msg)) => {
            eprintln!("❌ Validation error: {}", msg);
        }
        Err(e) => {
            eprintln!("❌ Database error: {}", e);
        }
    }
    
    Ok(())
}