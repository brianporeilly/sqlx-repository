//! # sqlx-repository
//!
//! [![Crates.io](https://img.shields.io/crates/v/sqlx-repository.svg)](https://crates.io/crates/sqlx-repository)
//! [![Documentation](https://docs.rs/sqlx-repository/badge.svg)](https://docs.rs/sqlx-repository)
//! [![License: MIT OR Apache-2.0](https://img.shields.io/crates/l/sqlx-repository.svg)](#license)
//!
//! A type-safe repository pattern implementation with derive macros for [sqlx](https://github.com/launchbadge/sqlx).
//!
//! This crate provides automatic CRUD operations, search functionality, soft delete support,
//! and comprehensive error handling through a simple derive macro.
//!
//! ## Features
//!
//! - **Type-safe derive macro** - Automatic repository generation from structs
//! - **Full CRUD operations** - Create, Read, Update, Delete with type safety  
//! - **Search & pagination** - Flexible search with filtering and pagination
//! - **Soft delete support** - Configurable soft delete with `deleted_at` field
//! - **Auto-generated types** - `CreateT` and `UpdateT` structs for mutations
//! - **PostgreSQL support** - Production-ready PostgreSQL backend
//! - **Clear error messages** - Helpful compile-time errors with examples
//!
//! ## Quick Start
//!
//! Add sqlx-repository to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! sqlx = { version = "0.7", features = ["postgres", "runtime-tokio-rustls", "macros", "chrono"] }
//! sqlx-repository = "0.1"
//! serde = { version = "1.0", features = ["derive"] }
//! chrono = { version = "0.4", features = ["serde"] }
//! ```
//!
//! Define your entity with the Repository derive:
//!
//! ```rust
//! use chrono::{DateTime, Utc};
//! use serde::{Deserialize, Serialize};
//! use sqlx_repository::Repository;
//!
//! #[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow, Repository)]
//! #[repository(table = "users")]
//! pub struct User {
//!     pub id: i32,
//!     pub name: String,
//!     pub email: String,
//!     pub created_at: DateTime<Utc>,
//!     pub updated_at: DateTime<Utc>,
//! }
//! ```
//!
//! Use your repository:
//!
//! ```rust,no_run
//! use sqlx::PgPool;
//! use sqlx_repository::SearchParams;
//! # use chrono::{DateTime, Utc};
//! # use serde::{Deserialize, Serialize};
//! # use sqlx_repository::Repository;
//! # #[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow, Repository)]
//! # #[repository(table = "users")]
//! # pub struct User {
//! #     pub id: i32,
//! #     pub name: String,
//! #     pub email: String,
//! #     pub created_at: DateTime<Utc>,
//! #     pub updated_at: DateTime<Utc>,
//! # }
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let pool = PgPool::connect("postgres://localhost/mydb").await?;
//!     let repo = UserRepository::new(pool);
//!     
//!     // Create
//!     let user = repo.create(CreateUser {
//!         name: "Alice".to_string(),
//!         email: "alice@example.com".to_string(),
//!     }).await?;
//!
//!     // Read
//!     let found = repo.find_by_id(user.id).await?;
//!
//!     // Update
//!     let updated = repo.update(user.id, UpdateUser {
//!         name: Some("Alice Smith".to_string()),
//!         email: None,
//!     }).await?;
//!
//!     // Search with pagination
//!     let results = repo.search(SearchParams {
//!         page: 0,
//!         per_page: 10,
//!         ..Default::default()
//!     }).await?;
//!     
//!     // Delete
//!     repo.delete(user.id).await?;
//!     
//!     Ok(())
//! }
//! ```
//!
//! ## Generated Code
//!
//! The `#[derive(Repository)]` macro automatically generates:
//!
//! ```rust,ignore
//! // Repository struct
//! pub struct UserRepository {
//!     pool: sqlx::PgPool,
//! }
//!
//! // Create type (required fields only)
//! #[derive(Debug, Clone, serde::Deserialize)]
//! pub struct CreateUser {
//!     pub name: String,
//!     pub email: String,
//! }
//!
//! // Update type (all fields optional)
//! #[derive(Debug, Clone, serde::Deserialize)]
//! pub struct UpdateUser {
//!     pub name: Option<String>,
//!     pub email: Option<String>,
//!     pub created_at: Option<DateTime<Utc>>,
//!     pub updated_at: Option<DateTime<Utc>>,
//! }
//!
//! // Full Repository trait implementation
//! impl Repository<User> for UserRepository { /* ... */ }
//! ```
//!
//! ## Soft Delete Support
//!
//! Enable soft delete functionality by adding the `#[repository(soft_delete)]` attribute
//! and including a `deleted_at` field:
//!
//! ```rust
//! use chrono::{DateTime, Utc};
//! use serde::{Deserialize, Serialize};
//! use sqlx_repository::Repository;
//!
//! #[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow, Repository)]
//! #[repository(table = "posts")]
//! #[repository(soft_delete)]  // Enable soft delete
//! pub struct Post {
//!     pub id: i32,
//!     pub title: String,
//!     pub content: String,
//!     pub created_at: DateTime<Utc>,
//!     pub updated_at: DateTime<Utc>,
//!     pub deleted_at: Option<DateTime<Utc>>,  // Required for soft delete
//! }
//! ```
//!
//! Soft deleted items are automatically excluded from queries:
//!
//! ```rust,no_run
//! # use chrono::{DateTime, Utc};
//! # use serde::{Deserialize, Serialize};
//! # use sqlx_repository::Repository;
//! # #[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow, Repository)]
//! # #[repository(table = "posts")]
//! # #[repository(soft_delete)]
//! # pub struct Post {
//! #     pub id: i32,
//! #     pub title: String,
//! #     pub content: String,
//! #     pub created_at: DateTime<Utc>,
//! #     pub updated_at: DateTime<Utc>,
//! #     pub deleted_at: Option<DateTime<Utc>>,
//! # }
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! # let pool = sqlx::PgPool::connect("postgres://localhost/mydb").await?;
//! let repo = PostRepository::new(pool);
//! let post_id = 1; // Example post ID
//!
//! // Soft delete (sets deleted_at timestamp)
//! repo.delete(post_id).await?;
//!
//! // Hard delete (permanently removes record)
//! repo.hard_delete(post_id).await?;
//!
//! // Search automatically excludes soft deleted items
//! let active_posts = repo.search(Default::default()).await?;
//! # Ok(())
//! # }
//! ```
//!
//! ## Supported Types
//!
//! | Category | Types | Example |
//! |----------|-------|---------|
//! | **Integers** | `i16`, `i32`, `i64`, `u16`, `u32`, `u64` | `pub age: i32` |
//! | **Floats** | `f32`, `f64` | `pub price: f64` |
//! | **Text** | `String` | `pub name: String` |
//! | **Boolean** | `bool` | `pub is_active: bool` |
//! | **Time** | `DateTime<Utc>`, `Date`, `Time` | `pub created_at: DateTime<Utc>` |
//! | **Optional** | `Option<T>` | `pub description: Option<String>` |
//! | **Collections** | `Vec<T>` | `pub tags: Vec<String>` |
//! | **UUID** | `Uuid` (with feature) | `pub uuid: Uuid` |
//!
//! For complex types, use JSON serialization with `String` fields.
//!
//! ## Error Handling
//!
//! The macro provides clear, actionable error messages with examples:
//!
//! ```compile_fail
//! #[derive(Repository)]
//! pub enum User {  // ❌ Enums not supported
//!     Active,
//!     Inactive,
//! }
//! ```
//!
//! ```text
//! error: Repository derive only supports structs, not enums.
//!        
//!        Example:
//!        #[derive(Repository)]
//!        #[repository(table = "users")]
//!        pub struct User {
//!            pub id: i32,
//!            pub name: String,
//!        }
//! ```
//!
//! ## Requirements
//!
//! - **Rust**: 1.70+
//! - **Database**: PostgreSQL 12+ (MySQL and SQLite coming in future versions)
//! - **Required derives**: `Debug`, `Clone`, `Serialize`, `Deserialize`, `sqlx::FromRow`, `Repository`
//! - **Required field**: `id: i32` (i64 and Uuid support coming soon)
//!
//! ## Examples
//!
//! See the [examples directory](https://github.com/brianporeilly/sqlx-repository/tree/main/examples) for complete working examples:
//!
//! - [Basic Usage](https://github.com/brianporeilly/sqlx-repository/blob/main/examples/basic_usage.rs) - CRUD operations and search
//! - [Soft Delete](https://github.com/brianporeilly/sqlx-repository/blob/main/examples/soft_delete.rs) - Soft delete functionality
//! - [Migrations](https://github.com/brianporeilly/sqlx-repository/blob/main/examples/migrations.rs) - Database setup and migrations
//!
//! ## License
//!
//! Licensed under either of:
//!
//! - Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
//! - MIT License ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)
//!
//! at your option.
//!
//! ## Features
//!
//! - **Type-safe CRUD operations**: Generated repository implementations with compile-time safety
//! - **Search and pagination**: Built-in support for filtering, sorting, and pagination
//! - **Soft deletes**: Optional soft delete functionality with restore capabilities
//! - **Async/await**: Full async support built on tokio and sqlx
//! - **Multi-database support**: PostgreSQL support now, MySQL and SQLite coming soon
//!
//! ## Repository Pattern
//!
//! The repository pattern provides a clean abstraction layer between your domain logic
//! and data access logic. This crate implements the pattern with:
//!
//! - Clear separation of concerns
//! - Consistent API across different entity types  
//! - Built-in common operations (CRUD, search, pagination)
//! - Extensibility for custom operations
//!
//! ## Examples
//!
//! See the `examples/` directory for comprehensive usage examples.

#![cfg_attr(docsrs, feature(doc_cfg))]
#![forbid(unsafe_code)]
#![warn(missing_docs, rust_2018_idioms)]

// Re-export key types for convenient usage
pub use error::{RepositoryError, RepositoryResult};
pub use repository::Repository;
pub use search::{RecordScope, SearchParams, SearchResult, SortOrder};

// Re-export derive macro when macros feature is enabled
#[cfg(feature = "macros")]
#[cfg_attr(docsrs, doc(cfg(feature = "macros")))]
pub use sqlx_repository_macros::Repository;

// Core modules
pub mod error;
pub mod repository;
pub mod search;

// Backend implementations
pub mod backends;

/// Common imports for sqlx-repository users
///
/// This prelude module contains the most commonly used types and traits
/// from the sqlx-repository crate, allowing for convenient glob imports.
///
/// # Examples
///
/// ```rust
/// use sqlx_repository::prelude::*;
/// 
/// // Now you have access to:
/// // - Repository trait
/// // - SearchParams, SearchResult, SortOrder, RecordScope
/// // - RepositoryError, RepositoryResult
/// // - Repository derive macro (if macros feature is enabled)
/// ```
pub mod prelude {
    //! Common imports for sqlx-repository users
    
    pub use crate::{Repository, SearchParams, SearchResult, SortOrder, RecordScope};
    pub use crate::{RepositoryError, RepositoryResult};
    
    #[cfg(feature = "macros")]
    #[cfg_attr(docsrs, doc(cfg(feature = "macros")))]
    pub use crate::Repository as RepositoryDerive;
    
    // Re-export commonly used async-trait for custom implementations
    pub use async_trait::async_trait;
}