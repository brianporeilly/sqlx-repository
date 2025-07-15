//! Migration example for sqlx-repository
//! 
//! This example shows how to create database tables for your repository structs
//! and run migrations using sqlx-migrate

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{PgPool};
use sqlx_repository::Repository;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow, Repository)]
#[repository(table = "users")]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow, Repository)]
#[repository(table = "posts")]
#[repository(soft_delete)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub content: String,
    pub author_id: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

async fn create_tables(pool: &PgPool) -> Result<(), sqlx::Error> {
    // Create users table
    sqlx::query(r#"
        CREATE TABLE IF NOT EXISTS users (
            id SERIAL PRIMARY KEY,
            name VARCHAR NOT NULL,
            email VARCHAR UNIQUE NOT NULL,
            created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
            updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
        )
    "#)
    .execute(pool)
    .await?;
    
    // Create posts table with soft delete support
    sqlx::query(r#"
        CREATE TABLE IF NOT EXISTS posts (
            id SERIAL PRIMARY KEY,
            title VARCHAR NOT NULL,
            content TEXT NOT NULL,
            author_id INTEGER NOT NULL REFERENCES users(id),
            created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
            updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
            deleted_at TIMESTAMPTZ
        )
    "#)
    .execute(pool)
    .await?;
    
    // Create indexes for better performance
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_posts_author_id ON posts(author_id)")
        .execute(pool)
        .await?;
    
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_posts_deleted_at ON posts(deleted_at)")
        .execute(pool)
        .await?;
    
    println!("✅ Created database tables");
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let database_url = "postgres://postgres:password@localhost/sqlx_repository_example";
    
    // Connect to database
    let pool = PgPool::connect(database_url).await?;
    
    // Create tables
    create_tables(&pool).await?;
    
    // Test the repositories
    let user_repo = UserRepository::new(pool.clone());
    let post_repo = PostRepository::new(pool.clone());
    
    // Create a user
    let user = user_repo.create(CreateUser {
        name: "John Doe".to_string(),
        email: "john@example.com".to_string(),
    }).await?;
    
    println!("👤 Created user: {} (ID: {})", user.name, user.id);
    
    // Create a post
    let post = post_repo.create(CreatePost {
        title: "My First Post".to_string(),
        content: "This is the content of my first post.".to_string(),
        author_id: user.id,
    }).await?;
    
    println!("📝 Created post: {} (ID: {})", post.title, post.id);
    
    // Demonstrate that foreign key constraint works
    let posts = sqlx::query_as::<_, (i32, String, i32)>(
        "SELECT id, title, author_id FROM posts WHERE author_id = $1"
    )
    .bind(user.id)
    .fetch_all(&pool)
    .await?;
    
    println!("🔗 Posts by user {}: {}", user.id, posts.len());
    
    println!("🎉 Migration example completed successfully!");
    
    Ok(())
}