//! Soft delete example for sqlx-repository
//! 
//! This example demonstrates:
//! - Enabling soft delete functionality
//! - Soft vs hard delete operations
//! - Querying with soft delete filtering

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use sqlx_repository::{Repository, SearchParams};

// Define a struct with soft delete enabled
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow, Repository)]
#[repository(table = "posts")]
#[repository(soft_delete)]  // Enable soft delete
pub struct Post {
    pub id: i32,
    pub title: String,
    pub content: String,
    pub author_id: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,  // Required for soft delete
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:password@localhost/sqlx_repository_dev".to_string());
    
    let pool = PgPool::connect(&database_url).await?;
    let post_repo = PostRepository::new(pool);
    
    // Create a new post
    let new_post = CreatePost {
        title: "Getting Started with Rust".to_string(),
        content: "Rust is a systems programming language...".to_string(),
        author_id: 1,
    };
    
    let post = post_repo.create(new_post).await?;
    println!("✅ Created post: {} (ID: {})", post.title, post.id);
    
    // Regular search (excludes soft deleted items by default)
    let search_result = post_repo.search(SearchParams::default()).await?;
    println!("📋 Active posts: {}", search_result.items.len());
    
    // Soft delete the post
    post_repo.delete(post.id).await?;
    println!("🗑️  Soft deleted post (deleted_at set)");
    
    // Search again - soft deleted post won't appear
    let search_result = post_repo.search(SearchParams::default()).await?;
    println!("📋 Active posts after soft delete: {}", search_result.items.len());
    
    // Try to find the soft deleted post by ID (returns None)
    match post_repo.find_by_id(post.id).await? {
        Some(_) => println!("🔍 Post found (unexpected!)"),
        None => println!("🔍 Post not found (soft deleted)"),
    }
    
    // Hard delete permanently removes the record
    let deleted = post_repo.hard_delete(post.id).await?;
    if deleted {
        println!("💀 Hard deleted post (permanently removed)");
    }
    
    Ok(())
}