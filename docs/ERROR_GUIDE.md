# Error Guide for sqlx-repository

This guide explains common compile-time errors you might encounter when using the `#[derive(Repository)]` macro and how to fix them.

## Common Compilation Errors

### 1. "Repository derive only supports structs, not enums"

**Error Message:**
```
error: Repository derive only supports structs, not enums.

       Example:
       #[derive(Repository)]
       #[repository(table = "users")]
       pub struct User {
           pub id: i32,
           pub name: String,
       }
```

**Cause:** You're trying to use `#[derive(Repository)]` on an enum or union type.

**Solution:** Repository derive only works on structs with named fields. Convert your enum to a struct:

```rust
// ❌ This won't work
#[derive(Repository)]
pub enum User {
    Active,
    Inactive,
}

// ✅ Use a struct instead
#[derive(Repository)]
#[repository(table = "users")]
pub struct User {
    pub id: i32,
    pub name: String,
    pub status: String,  // Use a field for the enum-like data
}
```

### 2. "Repository structs must have a primary key field named 'id: i32'"

**Error Message:**
```
error: Repository structs must have a primary key field named 'id: i32'.

       Example:
       #[derive(Repository)]
       #[repository(table = "users")]
       pub struct User {
           pub id: i32,  // ← Add this field
           pub name: String,
           pub email: String,
       }
```

**Cause:** Your struct is missing the required `id` field.

**Solution:** Add an `id: i32` field to your struct:

```rust
// ❌ Missing primary key
#[derive(Repository)]
#[repository(table = "users")]
pub struct User {
    pub name: String,
    pub email: String,
}

// ✅ Add the id field
#[derive(Repository)]
#[repository(table = "users")]
pub struct User {
    pub id: i32,        // Required primary key
    pub name: String,
    pub email: String,
}
```

### 3. "Primary key 'id' must be of type 'i32'"

**Error Message:**
```
error: Primary key 'id' must be of type 'i32', found 'String'.

       Currently supported: i32
       Future versions will support: i64, Uuid

       Example:
       pub struct User {
           pub id: i32,  // ← Must be i32
           pub name: String,
       }
```

**Cause:** Your `id` field is not of type `i32`.

**Solution:** Change your id field to `i32`:

```rust
// ❌ Wrong primary key type
#[derive(Repository)]
#[repository(table = "users")]
pub struct User {
    pub id: String,     // Won't work
    pub name: String,
}

// ✅ Use i32 for primary key
#[derive(Repository)]
#[repository(table = "users")]
pub struct User {
    pub id: i32,        // Correct type
    pub name: String,
}
```

### 4. "Soft delete enabled but no 'deleted_at' field found"

**Error Message:**
```
error: Soft delete enabled but no 'deleted_at' field found. Add 'pub deleted_at: Option<DateTime<Utc>>' to your struct
```

**Cause:** You enabled soft delete with `#[repository(soft_delete)]` but didn't add the required `deleted_at` field.

**Solution:** Add the `deleted_at` field:

```rust
use chrono::{DateTime, Utc};

// ❌ Missing deleted_at field
#[derive(Repository)]
#[repository(table = "users")]
#[repository(soft_delete)]
pub struct User {
    pub id: i32,
    pub name: String,
    // Missing deleted_at field
}

// ✅ Add the deleted_at field
#[derive(Repository)]
#[repository(table = "users")]
#[repository(soft_delete)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub deleted_at: Option<DateTime<Utc>>,  // Required for soft delete
}
```

### 5. "Unsupported field type"

**Error Message:**
```
error: Unsupported field type 'HashMap' for field 'metadata'.

       Supported types:
       - Integers: i16, i32, i64, u16, u32, u64
       - Floats: f32, f64
       - Text: String, &str
       - Boolean: bool
       - Time: DateTime<Utc>, Date, Time
       - Optional: Option<T> for any supported type T
       - Collections: Vec<T> for supported types T
       - UUID: Uuid (with uuid feature)

       For complex types, consider using JSON serialization with String fields.
```

**Cause:** You're using a complex type that's not supported by the repository pattern.

**Solution:** Use supported types or serialize complex data as JSON:

```rust
use std::collections::HashMap;
use serde_json;

// ❌ Unsupported complex type
#[derive(Repository)]
#[repository(table = "users")]
pub struct User {
    pub id: i32,
    pub metadata: HashMap<String, String>,  // Not supported
}

// ✅ Use JSON serialization with String
#[derive(Repository)]
#[repository(table = "users")]
pub struct User {
    pub id: i32,
    pub metadata_json: String,  // Store as JSON string
}

impl User {
    pub fn set_metadata(&mut self, metadata: HashMap<String, String>) {
        self.metadata_json = serde_json::to_string(&metadata).unwrap();
    }
    
    pub fn get_metadata(&self) -> HashMap<String, String> {
        serde_json::from_str(&self.metadata_json).unwrap_or_default()
    }
}
```

### 6. Missing Required Derives

**Error Message:**
```
error[E0277]: the trait bound `User: Debug` is not satisfied
```

**Cause:** The Repository derive requires certain standard derives to be present.

**Solution:** Add all required derives:

```rust
// ❌ Missing required derives
#[derive(Repository)]
#[repository(table = "users")]
pub struct User {
    pub id: i32,
    pub name: String,
}

// ✅ Add all required derives
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, sqlx::FromRow, Repository)]
#[repository(table = "users")]
pub struct User {
    pub id: i32,
    pub name: String,
}
```

## Best Practices

### 1. Complete Struct Template

Here's a complete template for a Repository struct:

```rust
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx_repository::Repository;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow, Repository)]
#[repository(table = "users")]
#[repository(soft_delete)]  // Optional
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,  // Required if using soft_delete
}
```

### 2. Supported Field Types

| Category | Types | Example |
|----------|-------|---------|
| **Integers** | `i16`, `i32`, `i64`, `u16`, `u32`, `u64` | `pub age: i32` |
| **Floats** | `f32`, `f64` | `pub price: f64` |
| **Text** | `String`, `&str` | `pub name: String` |
| **Boolean** | `bool` | `pub is_active: bool` |
| **Time** | `DateTime<Utc>`, `Date`, `Time` | `pub created_at: DateTime<Utc>` |
| **Optional** | `Option<T>` | `pub description: Option<String>` |
| **Collections** | `Vec<T>` | `pub tags: Vec<String>` |
| **UUID** | `Uuid` (with feature) | `pub uuid: Uuid` |

### 3. Table Name Conventions

If you don't specify a table name, it will be auto-generated from your struct name:

```rust
// Auto-generated table names:
pub struct User { ... }      // → "users"
pub struct BlogPost { ... }  // → "blog_posts" 
pub struct Category { ... }  // → "categories"

// Or specify explicitly:
#[repository(table = "custom_users")]
pub struct User { ... }
```

### 4. Feature Requirements

Make sure to enable the required features in your `Cargo.toml`:

```toml
[dependencies]
sqlx = { version = "0.7", features = ["postgres", "runtime-tokio-rustls", "macros", "chrono"] }
sqlx-repository = { version = "0.1", features = ["postgres"] }
serde = { version = "1.0", features = ["derive"] }
chrono = { version = "0.4", features = ["serde"] }
tokio = { version = "1.0", features = ["macros"] }
```

## Getting Help

If you encounter an error not covered in this guide:

1. **Check the error message** - Our macros provide detailed error messages with examples
2. **Review the examples** - Look at the `examples/` directory for working code
3. **Check the test cases** - The `tests/macro_tests/compile_pass/` directory has valid examples
4. **File an issue** - If you believe there's a bug or missing feature, file an issue on GitHub

## Error Prevention Checklist

Before using `#[derive(Repository)]`, make sure:

- [ ] Your type is a struct with named fields (not enum/union)
- [ ] You have an `id: i32` field
- [ ] You have all required derives: `Debug`, `Clone`, `Serialize`, `Deserialize`, `sqlx::FromRow`, `Repository`
- [ ] If using `#[repository(soft_delete)]`, you have `deleted_at: Option<DateTime<Utc>>`
- [ ] All field types are supported (see table above)
- [ ] You have the required features enabled in `Cargo.toml`