//! Test utilities for integration tests
//!
//! This module provides utilities for setting up test databases with proper migrations
//! and cleanup functionality.

use sqlx::{PgPool, migrate::MigrateDatabase, Postgres};
use std::env;
use std::sync::atomic::{AtomicU32, Ordering};

static TEST_COUNTER: AtomicU32 = AtomicU32::new(0);

pub fn get_unique_suffix() -> u32 {
    TEST_COUNTER.fetch_add(1, Ordering::SeqCst)
}

/// Database configuration for tests
pub struct TestDbConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub database: String,
}

impl Default for TestDbConfig {
    fn default() -> Self {
        Self {
            host: env::var("TEST_DB_HOST").unwrap_or_else(|_| "localhost".to_string()),
            port: env::var("TEST_DB_PORT")
                .unwrap_or_else(|_| "5433".to_string())
                .parse()
                .unwrap_or(5433),
            username: env::var("TEST_DB_USERNAME").unwrap_or_else(|_| "postgres".to_string()),
            password: env::var("TEST_DB_PASSWORD").unwrap_or_else(|_| "password".to_string()),
            database: env::var("TEST_DB_NAME").unwrap_or_else(|_| "sqlx_repository_test".to_string()),
        }
    }
}

impl TestDbConfig {
    pub fn database_url(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.database
        )
    }
}

/// Setup test database with migrations
pub async fn setup_test_db() -> PgPool {
    setup_test_db_with_config(TestDbConfig::default()).await
}

/// Setup test database with custom configuration
pub async fn setup_test_db_with_config(config: TestDbConfig) -> PgPool {
    let database_url = config.database_url();
    
    // Ensure database exists
    if !Postgres::database_exists(&database_url).await.unwrap_or(false) {
        Postgres::create_database(&database_url)
            .await
            .expect("Failed to create test database");
    }
    
    // Connect to database
    let pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to test database");
    
    // Run migrations
    run_migrations(&pool).await;
    
    // Clean up any existing test data to ensure test isolation
    cleanup_test_data(&pool).await;
    
    pool
}

/// Run migrations on the given database pool
pub async fn run_migrations(pool: &PgPool) {
    sqlx::migrate!("./migrations")
        .run(pool)
        .await
        .expect("Failed to run migrations");
}

/// Reset database by dropping and recreating all tables
pub async fn reset_test_db(pool: &PgPool) {
    // Drop all tables in reverse dependency order
    let _ = sqlx::query("DROP TABLE IF EXISTS posts CASCADE")
        .execute(pool)
        .await;
    
    let _ = sqlx::query("DROP TABLE IF EXISTS users CASCADE")
        .execute(pool)
        .await;
    
    // Re-run migrations to recreate tables
    run_migrations(pool).await;
}

/// Clean up test data while preserving schema and seed data
pub async fn cleanup_test_data(pool: &PgPool) {
    // Delete test data that might have been left from previous runs
    // Use LIKE patterns to match test data specifically, but preserve seed data
    let _ = sqlx::query("DELETE FROM posts WHERE title LIKE '%Test%' OR content LIKE '%test%'")
        .execute(pool)
        .await;
        
    // Only delete test users, not seed data (john@example.com, jane@example.com, bob@example.com)
    let _ = sqlx::query("DELETE FROM users WHERE (email LIKE '%test%' OR email LIKE 'alice.eng%' OR email LIKE 'bob.marketing%' OR email LIKE 'charlie.eng%' OR email LIKE 'count_test%' OR email LIKE 'delete_test%' OR email LIKE 'post_test%') AND email NOT IN ('john@example.com', 'jane@example.com', 'bob@example.com')")
        .execute(pool)
        .await;
}

/// Create a test database with a unique name for complete isolation
pub async fn create_isolated_test_db() -> (PgPool, String) {
    let config = TestDbConfig::default();
    let unique_db_name = format!("{}_{}", config.database, get_unique_suffix());
    
    let isolated_config = TestDbConfig {
        database: unique_db_name.clone(),
        ..config
    };
    
    let pool = setup_test_db_with_config(isolated_config).await;
    (pool, unique_db_name)
}

/// Drop a test database by name
pub async fn drop_test_db(database_name: &str) {
    let config = TestDbConfig::default();
    let admin_url = format!(
        "postgres://{}:{}@{}:{}/postgres",
        config.username, config.password, config.host, config.port
    );
    
    let admin_pool = PgPool::connect(&admin_url)
        .await
        .expect("Failed to connect to admin database");
    
    let _ = sqlx::query(&format!("DROP DATABASE IF EXISTS {}", database_name))
        .execute(&admin_pool)
        .await;
    
    admin_pool.close().await;
}