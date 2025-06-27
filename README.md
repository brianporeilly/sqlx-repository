# sqlx-repository

[![Crates.io](https://img.shields.io/crates/v/sqlx-repository.svg)](https://crates.io/crates/sqlx-repository)
[![Documentation](https://docs.rs/sqlx-repository/badge.svg)](https://docs.rs/sqlx-repository)
[![License](https://img.shields.io/crates/l/sqlx-repository.svg)](LICENSE-MIT)

A type-safe repository pattern implementation with derive macros for [SQLx](https://github.com/launchbadge/sqlx).

## Features

- **Type-safe CRUD operations**: Generated repository implementations with compile-time safety
- **Search and pagination**: Built-in support for filtering, sorting, and pagination
- **Soft deletes**: Optional soft delete functionality with restore capabilities
- **Async/await**: Full async support built on tokio and sqlx
- **Multi-database support**: PostgreSQL support now, MySQL and SQLite coming soon
- **Derive macros**: Automatic repository generation with minimal boilerplate
- **Extensible**: Easy to add custom methods to generated repositories

## Quick Start

Add this to your `Cargo.toml`:

```toml
[dependencies]
sqlx-repository = { version = "0.1", features = ["postgres"] }
sqlx = { version = "0.7", features = ["postgres", "runtime-tokio-rustls"] }
tokio = { version = "1.0", features = ["full"] }
```

Define your entity and derive the repository:

```rust
use sqlx_repository::prelude::*;
use sqlx::FromRow;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, RepositoryDerive)]
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
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub deleted_at: Option<chrono::DateTime<chrono::Utc>>,
}
```

Use the generated repository:

```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pool = sqlx::PgPool::connect("postgres://...").await?;
    let repo = UserRepository::new(pool);

    // Create a user
    let user = repo.create(CreateUser {
        name: "John Doe".to_string(),
        email: "john@example.com".to_string(),
        status: "active".to_string(),
        department: "engineering".to_string(),
    }).await?;

    // Find by ID
    let found = repo.find_by_id(user.id).await?;

    // Search with filters
    let mut filters = std::collections::HashMap::new();
    filters.insert("department".to_string(), "engineering".to_string());
    
    let results = repo.search(SearchParams {
        query: Some("John".to_string()),
        filters,
        page: 0,
        per_page: 10,
        ..Default::default()
    }).await?;

    println!("Found {} users", results.total_count);
    
    Ok(())
}
```

## Repository Features

### Automatic CRUD Operations

The derive macro generates complete CRUD implementations:

- `create(data)` - Create new entities
- `find_by_id(id)` - Find by primary key
- `find_all()` - Find all entities
- `update(id, data)` - Update existing entities
- `delete(id)` - Delete entities (soft or hard)
- `search(params)` - Advanced search with filtering and pagination

### Search and Pagination

Built-in search functionality with:

- **Text search**: Search across specified fields
- **Filtering**: Field-specific filters
- **Sorting**: Sort by any field, ascending or descending
- **Pagination**: Efficient offset-based pagination
- **Counting**: Total count with search filters applied

```rust
let results = repo.search(SearchParams {
    query: Some("john".to_string()),           // Search name/email fields
    filters: {
        let mut f = HashMap::new();
        f.insert("status".to_string(), "active".to_string());
        f
    },
    page: 0,                                   // First page (0-based)
    per_page: 20,                             // 20 items per page
    sort_by: Some("created_at".to_string()),  // Sort by creation date
    sort_order: SortOrder::Desc,              // Newest first
    scope: RecordScope::Active,               // Only non-deleted
}).await?;
```

### Soft Deletes

Enable soft deletes with the `soft_delete` attribute:

```rust
#[derive(RepositoryDerive)]
#[repository(table = "users")]
#[repository(soft_delete)]  // Enable soft deletes
pub struct User {
    // ... fields
    pub deleted_at: Option<chrono::DateTime<chrono::Utc>>,
}
```

Soft delete operations:

```rust
// Soft delete (sets deleted_at timestamp)
repo.delete(user_id).await?;

// Restore soft-deleted entity
repo.restore(user_id).await?;

// Permanently delete (removes from database)
repo.hard_delete(user_id).await?;

// Search deleted records
let deleted = repo.search(SearchParams {
    scope: RecordScope::Deleted,
    ..Default::default()
}).await?;
```

### Custom Repository Methods

Extend generated repositories with custom methods:

```rust
impl UserRepository {
    pub async fn find_by_email(&self, email: &str) -> RepositoryResult<Option<User>> {
        sqlx::query_as("SELECT * FROM users WHERE email = $1 AND deleted_at IS NULL")
            .bind(email)
            .fetch_optional(self.pool())
            .await
            .map_err(RepositoryError::from)
    }

    pub async fn find_active_users(&self) -> RepositoryResult<Vec<User>> {
        sqlx::query_as("SELECT * FROM users WHERE status = 'active' AND deleted_at IS NULL")
            .fetch_all(self.pool())
            .await
            .map_err(RepositoryError::from)
    }
}
```

## Configuration

### Repository Attributes

- `#[repository(table = "table_name")]` - Specify custom table name
- `#[repository(soft_delete)]` - Enable soft delete functionality
- `#[repository(searchable_fields(field1, field2))]` - Fields for text search
- `#[repository(filterable_fields(field1, field2))]` - Fields for filtering

### Feature Flags

- `postgres` - PostgreSQL support (default)
- `mysql` - MySQL support (coming soon)
- `sqlite` - SQLite support (coming soon)
- `macros` - Enable derive macros (default)
- `migrate` - Migration utilities (optional)

## Testing

Run the test suite:

```bash
# Unit tests (fast, no external dependencies)
make test-unit

# Integration tests (requires Docker)
make docker-up
make test-integration

# All tests
make test-all

# Benchmarks
make bench
```

### Database Setup for Testing

Start test databases with Docker:

```bash
make docker-up
```

Or manually:

```bash
docker-compose -f docker-compose.test.yml up -d
```

## Examples

See the [`examples/`](examples/) directory for comprehensive usage examples:

- [`basic_usage.rs`](examples/basic_usage.rs) - Complete CRUD operations walkthrough
- More examples coming soon...

Run examples:

```bash
# Set up database
export DATABASE_URL="postgres://test_user:test_pass@localhost:5432/test_db"

# Run example
cargo run --example basic_usage --features postgres,_rt-tokio
```

## Roadmap

### Phase 1: Production Ready (v0.1.0) ✅
- [x] Clean crate structure with separate proc-macro crate
- [x] Production-ready error handling with thiserror
- [x] Comprehensive unit tests and integration tests
- [x] Benchmarks and performance testing
- [x] Documentation and examples

### Phase 2: Multi-Database Support (v0.2.0)
- [ ] Database backend abstraction layer
- [ ] MySQL support with feature flags
- [ ] SQLite support with feature flags
- [ ] Cross-database compatibility tests

### Phase 3: Advanced Features (v0.3.0)
- [ ] Database-specific optimizations
- [ ] Migration generation utilities
- [ ] Advanced query building
- [ ] Connection pooling enhancements

## Contributing

Contributions are welcome! Please read our [contributing guidelines](CONTRIBUTING.md) for details on how to get started.

## License

This project is licensed under the MIT OR Apache-2.0 license.

- [MIT License](LICENSE-MIT)
- [Apache License, Version 2.0](LICENSE-APACHE)