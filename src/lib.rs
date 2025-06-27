//! # sqlx-repository
//!
//! A type-safe repository pattern implementation with derive macros for sqlx.
//!
//! This crate provides a clean, type-safe way to implement the repository pattern
//! with SQLx, featuring automatic CRUD operations, search functionality, and
//! support for soft deletes through derive macros.
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use sqlx_repository::{Repository, SearchParams};
//! 
//! #[derive(sqlx_repository::Repository)]
//! #[repository(table = "users")]
//! pub struct User {
//!     pub id: i32,
//!     pub name: String,
//!     pub email: String,
//! }
//! 
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let pool = sqlx::PgPool::connect("postgres://...").await?;
//! let repo = UserRepository::new(pool);
//! 
//! let user = repo.find_by_id(1).await?;
//! # Ok(())
//! # }
//! ```
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