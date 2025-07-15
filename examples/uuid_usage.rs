//! Example demonstrating UUID-based repository usage
//!
//! This example shows how to use UUID primary keys instead of auto-incrementing integers.
//! UUIDs are particularly useful for:
//! - Distributed systems where you need globally unique IDs
//! - Avoiding database round-trips for ID generation
//! - Better security (non-sequential IDs)
//! - Easier data merging across systems

use sqlx_repository::prelude::*;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// User entity with UUID primary key
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow, Repository)]
#[repository(table = "uuid_users")]
#[repository(soft_delete)]
#[repository(searchable_fields(name, email))]
#[repository(filterable_fields(status, department))]
pub struct UuidUser {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub status: String,
    pub department: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

/// Blog post entity with UUID primary key and UUID foreign key
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow, Repository)]
#[repository(table = "uuid_posts")]
#[repository(searchable_fields(title, content))]
#[repository(filterable_fields(published, user_id))]
pub struct UuidPost {
    pub id: Uuid,
    pub title: String,
    pub content: String,
    pub user_id: Uuid,  // Foreign key to UuidUser
    pub published: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🆔 UUID Repository Example");
    println!("==========================\n");

    // This example demonstrates the UUID repository pattern
    // but requires a running PostgreSQL database to execute
    
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:password@localhost:5433/sqlx_repository_test".to_string());

    println!("📋 Key Features of UUID Repositories:");
    println!("• Globally unique identifiers");
    println!("• Generated on the client side (no DB round-trip)");
    println!("• Better for distributed systems");
    println!("• Non-sequential for security");
    println!("• Same Repository API as integer IDs\n");

    // Show the generated types (this compiles but doesn't run without DB)
    println!("🔧 Generated Types:");
    println!("• UuidUserRepository - Repository implementation");
    println!("• CreateUuidUser - For new user creation");
    println!("• UpdateUuidUser - For partial updates\n");

    println!("📝 Example Usage (requires database connection):");
    println!("```rust");
    println!("// Connect to database");
    println!("let pool = sqlx::PgPool::connect(&database_url).await?;");
    println!("let user_repo = UuidUserRepository::new(pool.clone());");
    println!("let post_repo = UuidPostRepository::new(pool);");
    println!();
    println!("// Create a new user (UUID is generated automatically)");
    println!("let new_user = CreateUuidUser {{");
    println!("    name: \"Alice Johnson\".to_string(),");
    println!("    email: \"alice@example.com\".to_string(),");
    println!("    status: \"active\".to_string(),");
    println!("    department: \"engineering\".to_string(),");
    println!("}};");
    println!();
    println!("let user = user_repo.create(new_user).await?;");
    println!("println!(\"Created user with ID: {{}}\", user.id);");
    println!();
    println!("// Create a post linked to the user");
    println!("let new_post = CreateUuidPost {{");
    println!("    title: \"My First Post\".to_string(),");
    println!("    content: \"Hello, world!\".to_string(),");
    println!("    user_id: user.id,  // Use the UUID from the user");
    println!("    published: true,");
    println!("}};");
    println!();
    println!("let post = post_repo.create(new_post).await?;");
    println!("println!(\"Created post with ID: {{}}\", post.id);");
    println!();
    println!("// Find by UUID");
    println!("let found_user = user_repo.find_by_id(user.id).await?;");
    println!("if let Some(user) = found_user {{");
    println!("    println!(\"Found user: {{}}\", user.name);");
    println!("}}");
    println!("```\n");

    println!("🗄️  Required Database Schema:");
    println!("```sql");
    println!("-- Enable UUID extension");
    println!("CREATE EXTENSION IF NOT EXISTS \"uuid-ossp\";");
    println!();
    println!("-- Users table with UUID primary key");
    println!("CREATE TABLE uuid_users (");
    println!("    id UUID PRIMARY KEY,");
    println!("    name VARCHAR NOT NULL,");
    println!("    email VARCHAR UNIQUE NOT NULL,");
    println!("    status VARCHAR DEFAULT 'active',");
    println!("    department VARCHAR,");
    println!("    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),");
    println!("    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),");
    println!("    deleted_at TIMESTAMP WITH TIME ZONE");
    println!(");");
    println!();
    println!("-- Posts table with UUID primary and foreign keys");
    println!("CREATE TABLE uuid_posts (");
    println!("    id UUID PRIMARY KEY,");
    println!("    title VARCHAR NOT NULL,");
    println!("    content TEXT NOT NULL,");
    println!("    user_id UUID NOT NULL REFERENCES uuid_users(id),");
    println!("    published BOOLEAN DEFAULT FALSE,");
    println!("    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),");
    println!("    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()");
    println!(");");
    println!("```\n");

    println!("⚡ Advantages of UUID over Integer IDs:");
    println!("• No central coordination needed for ID generation");
    println!("• Can generate IDs offline");
    println!("• Better for data replication and merging");
    println!("• Harder to guess or enumerate");
    println!("• Globally unique across all systems\n");

    println!("⚠️  Considerations:");
    println!("• Larger storage size (16 bytes vs 4/8 bytes)");
    println!("• Slightly slower comparisons");
    println!("• Random UUID can hurt database performance vs sequential");
    println!("• Consider using UUID v7 for better DB performance\n");

    println!("🚀 To run this example with a real database:");
    println!("1. Start PostgreSQL with the UUID extension");
    println!("2. Create the required tables");
    println!("3. Set DATABASE_URL environment variable");
    println!("4. Run: cargo run --example uuid_usage --features uuid");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uuid_types_compile() {
        // Test that the UUID types compile correctly
        let _user_id: Uuid = Uuid::new_v4();
        
        let _create_user = CreateUuidUser {
            name: "Test User".to_string(),
            email: "test@example.com".to_string(),
            status: "active".to_string(),
            department: "test".to_string(),
        };
        
        let _update_user = UpdateUuidUser {
            name: Some("Updated User".to_string()),
            email: None,
            status: None,
            department: Some("updated".to_string()),
        };
        
        // Verify the types exist and have the expected structure
        assert_eq!(std::mem::size_of::<Uuid>(), 16); // UUID is 16 bytes
    }

    #[test]
    fn test_uuid_generation() {
        // Test UUID generation
        let id1 = Uuid::new_v4();
        let id2 = Uuid::new_v4();
        
        // UUIDs should be unique
        assert_ne!(id1, id2);
        
        // UUIDs should be valid
        assert_eq!(id1.get_version(), Some(uuid::Version::Random));
        assert_eq!(id2.get_version(), Some(uuid::Version::Random));
    }
}