# sqlx-repository

[![Crates.io](https://img.shields.io/crates/v/sqlx-repository.svg)](https://crates.io/crates/sqlx-repository)
[![Documentation](https://docs.rs/sqlx-repository/badge.svg)](https://docs.rs/sqlx-repository)
[![License: MIT OR Apache-2.0](https://img.shields.io/crates/l/sqlx-repository.svg)](#license)
[![CI](https://github.com/brianporeilly/sqlx-repository/workflows/CI/badge.svg)](https://github.com/brianporeilly/sqlx-repository/actions/workflows/ci.yml)
[![codecov](https://codecov.io/gh/brianporeilly/sqlx-repository/branch/main/graph/badge.svg)](https://codecov.io/gh/brianporeilly/sqlx-repository)

> **Early Development**: This crate is currently in early development. APIs may change before v1.0.0.

A type-safe repository pattern implementation with derive macros for [sqlx](https://github.com/launchbadge/sqlx), providing automatic CRUD operations, search functionality, and comprehensive error handling.

## Features

- **Type-safe derive macro** - Automatic repository generation from structs
- **Full CRUD operations** - Create, Read, Update, Delete with type safety
- **Search & pagination** - Flexible search with filtering and pagination
- **Soft delete support** - Configurable soft delete with `deleted_at` field
- **Auto-generated types** - `CreateT` and `UpdateT` structs for mutations
- **PostgreSQL support** - Production-ready PostgreSQL backend
- **Clear error messages** - Helpful compile-time errors with examples

## Quick Start

Add to your `Cargo.toml`:

```toml
[dependencies]
sqlx = { version = "0.7", features = ["postgres", "runtime-tokio-rustls", "macros", "chrono"] }
sqlx-repository = "0.1"
serde = { version = "1.0", features = ["derive"] }
chrono = { version = "0.4", features = ["serde"] }
tokio = { version = "1.0", features = ["macros"] }
```

Define your entity:

```rust
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
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
```

Use your repository:

```rust
use sqlx::PgPool;
use sqlx_repository::SearchParams;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pool = PgPool::connect("postgres://localhost/mydb").await?;
    let repo = UserRepository::new(pool);
    
    // Create
    let user = repo.create(CreateUser {
        name: "Alice".to_string(),
        email: "alice@example.com".to_string(),
    }).await?;
    
    // Read
    let found = repo.find_by_id(user.id).await?;
    
    // Update
    let updated = repo.update(user.id, UpdateUser {
        name: Some("Alice Smith".to_string()),
        email: None,
    }).await?;
    
    // Search with pagination
    let results = repo.search(SearchParams {
        page: 0,
        per_page: 10,
        ..Default::default()
    }).await?;
    
    // Delete
    repo.delete(user.id).await?;
    
    Ok(())
}
```

## Generated Code

The `#[derive(Repository)]` macro automatically generates:

```rust
// Repository struct
pub struct UserRepository {
    pool: sqlx::PgPool,
}

// Create type (required fields only)
#[derive(Debug, Clone, serde::Deserialize)]
pub struct CreateUser {
    pub name: String,
    pub email: String,
}

// Update type (all fields optional)
#[derive(Debug, Clone, serde::Deserialize)]
pub struct UpdateUser {
    pub name: Option<String>,
    pub email: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

// Full Repository trait implementation with all CRUD operations
impl Repository<User> for UserRepository { /* ... */ }
```

## Soft Delete Support

Enable soft delete functionality:

```rust
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow, Repository)]
#[repository(table = "posts")]
#[repository(soft_delete)]  // Enable soft delete
pub struct Post {
    pub id: i32,
    pub title: String,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,  // Required for soft delete
}
```

Soft deleted items are automatically excluded from queries:

```rust
// Soft delete (sets deleted_at timestamp)
repo.delete(post_id).await?;

// Hard delete (permanently removes record)
repo.hard_delete(post_id).await?;

// Search automatically excludes soft deleted items
let active_posts = repo.search(SearchParams::default()).await?;
```

## Error Handling

The macro provides clear, actionable error messages:

```rust
// L This will give a helpful error:
#[derive(Repository)]
pub enum User {  // Enums not supported
    Active,
    Inactive,
}

// Error: Repository derive only supports structs, not enums.
//        
//        Example:
//        #[derive(Repository)]
//        #[repository(table = "users")]
//        pub struct User {
//            pub id: i32,
//            pub name: String,
//        }
```

See the [Error Guide](docs/ERROR_GUIDE.md) for complete troubleshooting.

## Examples

- [Basic Usage](examples/basic_usage.rs) - CRUD operations and search
- [Soft Delete](examples/soft_delete.rs) - Soft delete functionality  
- [Migrations](examples/migrations.rs) - Database setup and migrations

Run examples:
```bash
# Start database
make docker-up

# Run basic example
cargo run --example basic_usage

# Run with custom database URL
DATABASE_URL=postgres://user:pass@host/db cargo run --example basic_usage
```

## Development

```bash
# Setup development environment
make dev

# Run all tests (no database required)
make test

# Run with database integration tests
make test-all

# Watch for changes
make watch

# View all available commands
make help
```

## Requirements

- **Rust**: 1.70+ 
- **Database**: PostgreSQL 12+
- **Required derives**: `Debug`, `Clone`, `Serialize`, `Deserialize`, `sqlx::FromRow`, `Repository`
- **Required field**: `id: i32`