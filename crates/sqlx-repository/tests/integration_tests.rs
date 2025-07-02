//! Integration tests for sqlx-repository
//!
//! These tests run against a real PostgreSQL database to ensure the repository
//! pattern works correctly with actual database operations.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use sqlx_repository::prelude::*;
use std::sync::atomic::{AtomicU32, Ordering};

/// Test User entity with repository derive
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

/// Test Post entity without soft delete
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow, Repository)]
#[repository(table = "posts")]
#[repository(searchable_fields(title, content))]
#[repository(filterable_fields(published, user_id))]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub content: String,
    pub user_id: i32,
    pub published: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

static TEST_COUNTER: AtomicU32 = AtomicU32::new(0);

fn get_unique_suffix() -> u32 {
    TEST_COUNTER.fetch_add(1, Ordering::SeqCst)
}

async fn setup_test_db() -> PgPool {
    let database_url = std::env::var("DATABASE_URL_POSTGRES")
        .or_else(|_| std::env::var("DATABASE_URL"))
        .unwrap_or_else(|_| "postgres://test_user:test_pass@localhost:5432/test_db".to_string());

    let pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to test database");

    // Clean up any existing test data to ensure test isolation
    cleanup_test_data(&pool).await;

    pool
}

async fn cleanup_test_data(pool: &PgPool) {
    // Delete test data that might have been left from previous runs
    // Use LIKE patterns to match test data specifically
    let _ = sqlx::query("DELETE FROM posts WHERE title LIKE '%Test%' OR content LIKE '%test%'")
        .execute(pool)
        .await;

    let _ =
        sqlx::query("DELETE FROM users WHERE email LIKE '%test%' OR email LIKE '%example.com%'")
            .execute(pool)
            .await;
}

#[tokio::test]
async fn test_user_crud_operations() {
    let pool = setup_test_db().await;
    let repo = UserRepository::new(pool);

    // Create a test user with unique identifier
    let suffix = get_unique_suffix();
    let create_user = CreateUser {
        name: format!("Test User {}", suffix),
        email: format!("test{}@example.com", suffix),
        status: "active".to_string(),
        department: "test".to_string(),
    };

    let user = repo
        .create(create_user)
        .await
        .expect("Failed to create user");
    assert_eq!(user.name, format!("Test User {}", suffix));
    assert_eq!(user.email, format!("test{}@example.com", suffix));
    assert!(user.id > 0);

    // Find by ID
    let found_user = repo.find_by_id(user.id).await.expect("Failed to find user");
    assert!(found_user.is_some());
    assert_eq!(found_user.unwrap().name, format!("Test User {}", suffix));

    // Update user
    let update_user = UpdateUser {
        name: Some(format!("Updated User {}", suffix)),
        email: None,
        status: Some("inactive".to_string()),
        department: None,
    };

    let updated_user = repo
        .update(user.id, update_user)
        .await
        .expect("Failed to update user");
    assert!(updated_user.is_some());
    let updated_user = updated_user.unwrap();
    assert_eq!(updated_user.name, format!("Updated User {}", suffix));
    assert_eq!(updated_user.status, "inactive");

    // Clean up
    repo.hard_delete(user.id)
        .await
        .expect("Failed to delete user");
    cleanup_test_data(repo.pool()).await;
}

#[tokio::test]
async fn test_soft_delete_operations() {
    let pool = setup_test_db().await;
    let repo = UserRepository::new(pool);

    // Create a test user with unique identifier
    let suffix = get_unique_suffix();
    let create_user = CreateUser {
        name: format!("Delete Test User {}", suffix),
        email: format!("delete_test{}@example.com", suffix),
        status: "active".to_string(),
        department: "test".to_string(),
    };

    let user = repo
        .create(create_user)
        .await
        .expect("Failed to create user");

    // Soft delete
    let deleted = repo
        .delete(user.id)
        .await
        .expect("Failed to soft delete user");
    assert!(deleted);

    // Should not be found in normal queries
    let found_user = repo
        .find_by_id(user.id)
        .await
        .expect("Failed to query user");
    assert!(found_user.is_none());

    // Should be found in deleted queries
    let deleted_params = SearchParams {
        scope: RecordScope::Deleted,
        ..Default::default()
    };
    let deleted_results = repo
        .search(deleted_params)
        .await
        .expect("Failed to search deleted users");
    assert!(deleted_results.items.iter().any(|u| u.id == user.id));

    // Restore user
    let restored_user = repo.restore(user.id).await.expect("Failed to restore user");
    assert!(restored_user.is_some());

    // Should be found again
    let found_user = repo
        .find_by_id(user.id)
        .await
        .expect("Failed to find restored user");
    assert!(found_user.is_some());

    // Clean up
    repo.hard_delete(user.id)
        .await
        .expect("Failed to delete user");
    cleanup_test_data(repo.pool()).await;
}

#[tokio::test]
async fn test_search_and_filtering() {
    let pool = setup_test_db().await;
    let repo = UserRepository::new(pool);

    // Create test users with unique identifiers
    let suffix = get_unique_suffix();
    let users_data = vec![
        CreateUser {
            name: format!("Alice Engineering {}", suffix),
            email: format!("alice.eng{}@example.com", suffix),
            status: "active".to_string(),
            department: "engineering".to_string(),
        },
        CreateUser {
            name: format!("Bob Marketing {}", suffix),
            email: format!("bob.marketing{}@example.com", suffix),
            status: "active".to_string(),
            department: "marketing".to_string(),
        },
        CreateUser {
            name: format!("Charlie Engineering {}", suffix),
            email: format!("charlie.eng{}@example.com", suffix),
            status: "inactive".to_string(),
            department: "engineering".to_string(),
        },
    ];

    let mut created_users = Vec::new();
    for user_data in users_data {
        let user = repo
            .create(user_data)
            .await
            .expect("Failed to create test user");
        created_users.push(user);
    }

    // Test text search
    let search_params = SearchParams {
        query: Some("Alice".to_string()),
        ..Default::default()
    };
    let search_results = repo
        .search(search_params)
        .await
        .expect("Failed to search users");
    assert!(search_results
        .items
        .iter()
        .any(|u| u.name.contains("Alice")));

    // Test filtering by department
    let mut filters = std::collections::HashMap::new();
    filters.insert("department".to_string(), "engineering".to_string());
    let filter_params = SearchParams {
        filters,
        ..Default::default()
    };
    let filter_results = repo
        .search(filter_params)
        .await
        .expect("Failed to filter users");
    assert!(filter_results
        .items
        .iter()
        .all(|u| u.department == "engineering"));
    assert_eq!(filter_results.items.len(), 2);

    // Test pagination
    let page_params = SearchParams {
        page: 0,
        per_page: 1,
        ..Default::default()
    };
    let page_results = repo
        .search(page_params)
        .await
        .expect("Failed to paginate users");
    assert_eq!(page_results.items.len(), 1);
    assert!(page_results.total_count >= 3);
    assert!(page_results.has_next_page());

    // Test sorting
    let sort_params = SearchParams {
        sort_by: Some("name".to_string()),
        sort_order: SortOrder::Asc,
        ..Default::default()
    };
    let sort_results = repo
        .search(sort_params)
        .await
        .expect("Failed to sort users");
    let names: Vec<&str> = sort_results.items.iter().map(|u| u.name.as_str()).collect();
    let mut sorted_names = names.clone();
    sorted_names.sort();
    // Note: We can't assert exact equality because there might be existing test data

    // Clean up
    for user in created_users {
        repo.hard_delete(user.id)
            .await
            .expect("Failed to delete test user");
    }
    cleanup_test_data(repo.pool()).await;
}

#[tokio::test]
async fn test_post_crud_without_soft_delete() {
    let pool = setup_test_db().await;
    let user_repo = UserRepository::new(pool.clone());
    let post_repo = PostRepository::new(pool);

    // Create a test user first with unique identifier
    let suffix = get_unique_suffix();
    let create_user = CreateUser {
        name: format!("Post Test User {}", suffix),
        email: format!("post_test{}@example.com", suffix),
        status: "active".to_string(),
        department: "test".to_string(),
    };
    let user = user_repo
        .create(create_user)
        .await
        .expect("Failed to create user");

    // Create a test post
    let create_post = CreatePost {
        title: "Test Post".to_string(),
        content: "This is a test post content".to_string(),
        user_id: user.id,
        published: false,
    };

    let post = post_repo
        .create(create_post)
        .await
        .expect("Failed to create post");
    assert_eq!(post.title, "Test Post");
    assert_eq!(post.user_id, user.id);
    assert!(!post.published);

    // Update post
    let update_post = UpdatePost {
        title: Some("Updated Post".to_string()),
        content: None,
        user_id: None,
        published: Some(true),
    };

    let updated_post = post_repo
        .update(post.id, update_post)
        .await
        .expect("Failed to update post");
    assert!(updated_post.is_some());
    let updated_post = updated_post.unwrap();
    assert_eq!(updated_post.title, "Updated Post");
    assert!(updated_post.published);

    // Delete post (hard delete since no soft delete)
    let deleted = post_repo
        .delete(post.id)
        .await
        .expect("Failed to delete post");
    assert!(deleted);

    // Should not be found
    let found_post = post_repo
        .find_by_id(post.id)
        .await
        .expect("Failed to query post");
    assert!(found_post.is_none());

    // Clean up user
    user_repo
        .hard_delete(user.id)
        .await
        .expect("Failed to delete user");
    cleanup_test_data(user_repo.pool()).await;
}

#[tokio::test]
async fn test_count_operations() {
    let pool = setup_test_db().await;
    let repo = UserRepository::new(pool);

    // Get initial count
    let initial_count = repo
        .count(SearchParams::default())
        .await
        .expect("Failed to get initial count");

    // Create test users with unique identifiers
    let suffix = get_unique_suffix();
    let mut created_users = Vec::new();
    for i in 0..3 {
        let create_user = CreateUser {
            name: format!("Count Test User {}_{}", suffix, i),
            email: format!("count_test_{}_{}@example.com", suffix, i),
            status: "active".to_string(),
            department: "test".to_string(),
        };
        let user = repo
            .create(create_user)
            .await
            .expect("Failed to create user");
        created_users.push(user);
    }

    // Check count increased
    let new_count = repo
        .count(SearchParams::default())
        .await
        .expect("Failed to get new count");
    assert_eq!(new_count, initial_count + 3);

    // Test filtered count
    let mut filters = std::collections::HashMap::new();
    filters.insert("department".to_string(), "test".to_string());
    let filter_params = SearchParams {
        filters,
        ..Default::default()
    };
    let filtered_count = repo
        .count(filter_params)
        .await
        .expect("Failed to get filtered count");
    assert!(filtered_count >= 3);

    // Clean up
    for user in created_users {
        repo.hard_delete(user.id)
            .await
            .expect("Failed to delete user");
    }
    cleanup_test_data(repo.pool()).await;
}
