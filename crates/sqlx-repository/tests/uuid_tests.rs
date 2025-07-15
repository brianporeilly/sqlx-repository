//! UUID-based repository tests
//!
//! These tests verify that UUID primary keys work correctly with the repository pattern.

mod test_utils;

use sqlx_repository::prelude::*;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use test_utils::*;
use uuid::Uuid;

/// Test entity with UUID primary key
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

/// Test entity with UUID, no soft delete
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow, Repository)]
#[repository(table = "uuid_posts")]
#[repository(searchable_fields(title, content))]
#[repository(filterable_fields(published))]
pub struct UuidPost {
    pub id: Uuid,
    pub title: String,
    pub content: String,
    pub user_id: Uuid,
    pub published: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

async fn setup_uuid_test_db() -> sqlx::PgPool {
    let pool = setup_test_db().await;
    
    // Create UUID-specific tables for testing
    let _ = sqlx::query("DROP TABLE IF EXISTS uuid_posts CASCADE")
        .execute(&pool)
        .await;
    
    let _ = sqlx::query("DROP TABLE IF EXISTS uuid_users CASCADE")
        .execute(&pool)
        .await;
    
    // Create UUID users table
    sqlx::query(r#"
        CREATE TABLE uuid_users (
            id UUID PRIMARY KEY,
            name VARCHAR NOT NULL,
            email VARCHAR UNIQUE NOT NULL,
            status VARCHAR DEFAULT 'active',
            department VARCHAR,
            created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
            updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
            deleted_at TIMESTAMP WITH TIME ZONE
        )
    "#)
    .execute(&pool)
    .await
    .expect("Failed to create uuid_users table");
    
    // Create UUID posts table
    sqlx::query(r#"
        CREATE TABLE uuid_posts (
            id UUID PRIMARY KEY,
            title VARCHAR NOT NULL,
            content TEXT NOT NULL,
            user_id UUID NOT NULL REFERENCES uuid_users(id),
            published BOOLEAN DEFAULT FALSE,
            created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
            updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
        )
    "#)
    .execute(&pool)
    .await
    .expect("Failed to create uuid_posts table");
    
    pool
}

#[tokio::test]
async fn test_uuid_user_crud_operations() {
    let pool = setup_uuid_test_db().await;
    let repo = UuidUserRepository::new(pool);
    
    // Create a test user with UUID
    let create_user = CreateUuidUser {
        name: "UUID Test User".to_string(),
        email: "uuid_test@example.com".to_string(),
        status: "active".to_string(),
        department: "engineering".to_string(),
    };
    
    let user = repo.create(create_user).await.expect("Failed to create UUID user");
    assert_eq!(user.name, "UUID Test User");
    assert_eq!(user.email, "uuid_test@example.com");
    
    // Verify the ID is a valid UUID
    assert!(user.id.to_string().len() == 36); // Standard UUID string length
    
    // Find by ID
    let found_user = repo.find_by_id(user.id).await.expect("Failed to find UUID user");
    assert!(found_user.is_some());
    assert_eq!(found_user.unwrap().name, "UUID Test User");
    
    // Update user
    let update_user = UpdateUuidUser {
        name: Some("Updated UUID User".to_string()),
        email: None,
        status: Some("inactive".to_string()),
        department: None,
    };
    
    let updated_user = repo.update(user.id, update_user).await.expect("Failed to update UUID user");
    assert!(updated_user.is_some());
    let updated_user = updated_user.unwrap();
    assert_eq!(updated_user.name, "Updated UUID User");
    assert_eq!(updated_user.status, "inactive");
    assert_eq!(updated_user.id, user.id); // ID should remain the same
    
    // Clean up
    repo.hard_delete(user.id).await.expect("Failed to delete UUID user");
}

#[tokio::test]
async fn test_uuid_soft_delete_operations() {
    let pool = setup_uuid_test_db().await;
    let repo = UuidUserRepository::new(pool);
    
    // Create a test user
    let create_user = CreateUuidUser {
        name: "UUID Soft Delete Test".to_string(),
        email: "uuid_soft_delete@example.com".to_string(),
        status: "active".to_string(),
        department: "test".to_string(),
    };
    
    let user = repo.create(create_user).await.expect("Failed to create UUID user");
    
    // Soft delete
    let deleted = repo.delete(user.id).await.expect("Failed to soft delete UUID user");
    assert!(deleted);
    
    // Should not be found in normal queries
    let found_user = repo.find_by_id(user.id).await.expect("Failed to query UUID user");
    assert!(found_user.is_none());
    
    // Should be found in deleted queries
    let deleted_params = SearchParams {
        scope: RecordScope::Deleted,
        ..Default::default()
    };
    let deleted_results = repo.search(deleted_params).await.expect("Failed to search deleted UUID users");
    assert!(deleted_results.items.iter().any(|u| u.id == user.id));
    
    // Restore user
    let restored_user = repo.restore(user.id).await.expect("Failed to restore UUID user");
    assert!(restored_user.is_some());
    
    // Should be found again
    let found_user = repo.find_by_id(user.id).await.expect("Failed to find restored UUID user");
    assert!(found_user.is_some());
    
    // Clean up
    repo.hard_delete(user.id).await.expect("Failed to delete UUID user");
}

#[tokio::test]
async fn test_uuid_post_crud_without_soft_delete() {
    let pool = setup_uuid_test_db().await;
    let user_repo = UuidUserRepository::new(pool.clone());
    let post_repo = UuidPostRepository::new(pool);
    
    // Create a test user first
    let create_user = CreateUuidUser {
        name: "UUID Post Test User".to_string(),
        email: "uuid_post_test@example.com".to_string(),
        status: "active".to_string(),
        department: "test".to_string(),
    };
    let user = user_repo.create(create_user).await.expect("Failed to create UUID user");
    
    // Create a test post
    let create_post = CreateUuidPost {
        title: "UUID Test Post".to_string(),
        content: "This is a test post with UUID".to_string(),
        user_id: user.id,
        published: false,
    };
    
    let post = post_repo.create(create_post).await.expect("Failed to create UUID post");
    assert_eq!(post.title, "UUID Test Post");
    assert_eq!(post.user_id, user.id);
    assert!(!post.published);
    
    // Verify the post ID is a valid UUID
    assert!(post.id.to_string().len() == 36);
    
    // Update post
    let update_post = UpdateUuidPost {
        title: Some("Updated UUID Post".to_string()),
        content: None,
        user_id: None,
        published: Some(true),
    };
    
    let updated_post = post_repo.update(post.id, update_post).await.expect("Failed to update UUID post");
    assert!(updated_post.is_some());
    let updated_post = updated_post.unwrap();
    assert_eq!(updated_post.title, "Updated UUID Post");
    assert!(updated_post.published);
    assert_eq!(updated_post.id, post.id); // ID should remain the same
    
    // Delete post (hard delete since no soft delete)
    let deleted = post_repo.delete(post.id).await.expect("Failed to delete UUID post");
    assert!(deleted);
    
    // Should not be found
    let found_post = post_repo.find_by_id(post.id).await.expect("Failed to query UUID post");
    assert!(found_post.is_none());
    
    // Clean up user
    user_repo.hard_delete(user.id).await.expect("Failed to delete UUID user");
}

#[tokio::test]
async fn test_uuid_search_and_filtering() {
    let pool = setup_uuid_test_db().await;
    let repo = UuidUserRepository::new(pool);
    
    // Create test users
    let users_data = vec![
        CreateUuidUser {
            name: "UUID Alice Engineering".to_string(),
            email: "uuid_alice_eng@example.com".to_string(),
            status: "active".to_string(),
            department: "engineering".to_string(),
        },
        CreateUuidUser {
            name: "UUID Bob Marketing".to_string(),
            email: "uuid_bob_marketing@example.com".to_string(),
            status: "active".to_string(),
            department: "marketing".to_string(),
        },
        CreateUuidUser {
            name: "UUID Charlie Engineering".to_string(),
            email: "uuid_charlie_eng@example.com".to_string(),
            status: "inactive".to_string(),
            department: "engineering".to_string(),
        },
    ];
    
    let mut created_users = Vec::new();
    for user_data in users_data {
        let user = repo.create(user_data).await.expect("Failed to create UUID test user");
        created_users.push(user);
    }
    
    // Test text search
    let search_params = SearchParams {
        query: Some("UUID Alice".to_string()),
        ..Default::default()
    };
    let search_results = repo.search(search_params).await.expect("Failed to search UUID users");
    assert!(search_results.items.iter().any(|u| u.name.contains("UUID Alice")));
    
    // Test filtering by department
    let mut filters = std::collections::HashMap::new();
    filters.insert("department".to_string(), "engineering".to_string());
    let filter_params = SearchParams {
        filters,
        ..Default::default()
    };
    let filter_results = repo.search(filter_params).await.expect("Failed to filter UUID users");
    assert!(filter_results.items.iter().all(|u| u.department == "engineering"));
    assert_eq!(filter_results.items.len(), 2);
    
    // Test pagination
    let page_params = SearchParams {
        page: 0,
        per_page: 1,
        ..Default::default()
    };
    let page_results = repo.search(page_params).await.expect("Failed to paginate UUID users");
    assert_eq!(page_results.items.len(), 1);
    assert!(page_results.has_next_page());
    
    // Clean up
    for user in created_users {
        repo.hard_delete(user.id).await.expect("Failed to delete UUID test user");
    }
}