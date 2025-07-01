# ADR-001: Phase 1 Core Repository Pattern Architecture

**Status**: Proposed  
**Date**: 2025-06-28  
**Deciders**: Core Team  
**Supersedes**: N/A  

## Context

We are implementing the foundation for `sqlx-repository`, a Rust crate that provides a repository pattern framework built on sqlx. Phase 1 (v0.1.0) focuses on establishing the core repository pattern with PostgreSQL support, basic CRUD operations, search capabilities, and soft delete functionality.

## Problem Statement

Modern Rust web applications require:
1. **Type-safe database operations** without sacrificing performance
2. **Standardized patterns** for common database operations (CRUD, search, pagination)
3. **Compile-time guarantees** for database schema alignment
4. **Reduced boilerplate** while maintaining explicit control
5. **Foundation for advanced features** (audit logging, relationships, migrations)

Current solutions either sacrifice type safety (diesel's runtime queries) or require extensive boilerplate (raw sqlx).

## Decision

### Architecture Overview

We will implement a **derive macro-based repository pattern** with the following core components:

```rust
#[derive(Repository)]
#[repository(table = "users")]
pub struct User {
    #[repository(primary_key)]
    pub id: i32,
    pub email: String,
    pub name: String,
    #[repository(soft_delete)]
    pub deleted_at: Option<DateTime<Utc>>,
}
```

### Core Design Principles

1. **Derive Macro Approach**: Generate repository implementations at compile-time
2. **Type Safety First**: Leverage Rust's type system for database schema validation
3. **PostgreSQL Foundation**: Start with solid PostgreSQL support, expand later
4. **Explicit Configuration**: Use attributes for customization rather than conventions
5. **Performance Parity**: Maintain performance equivalent to hand-written sqlx code

### Component Architecture

#### 1. Repository Trait & Implementation

```rust
#[async_trait]
pub trait Repository<T> {
    type Error;
    
    async fn create(&self, entity: &T) -> Result<T, Self::Error>;
    async fn find_by_id(&self, id: i32) -> Result<Option<T>, Self::Error>;
    async fn update(&self, entity: &T) -> Result<T, Self::Error>;
    async fn delete(&self, id: i32) -> Result<(), Self::Error>;
    async fn search(&self, params: SearchParams) -> Result<SearchResult<T>, Self::Error>;
}
```

#### 2. Backend Abstraction

```rust
pub trait DatabaseBackend {
    async fn execute_query(&self, query: &str, params: &[&dyn Any]) -> Result<QueryResult, Error>;
    fn generate_insert_sql(&self, table: &str, fields: &[&str]) -> String;
    fn generate_select_sql(&self, table: &str, conditions: &[Condition]) -> String;
    // ... other query generation methods
}
```

#### 3. Search & Pagination

```rust
pub struct SearchParams {
    pub filters: Vec<Filter>,
    pub sort: Option<Sort>,
    pub page: Option<u32>,
    pub limit: Option<u32>,
}

pub struct SearchResult<T> {
    pub items: Vec<T>,
    pub total_count: u64,
    pub page: u32,
    pub total_pages: u32,
}
```

#### 4. Error Handling

```rust
#[derive(Debug, thiserror::Error)]
pub enum RepositoryError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    
    #[error("Validation error: {0}")]
    Validation(String),
    
    #[error("Entity not found")]
    NotFound,
    
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
}
```

### Implementation Strategy

#### Phase 1.1: Core Infrastructure (Week 1)
- [ ] Crate structure setup (`sqlx-repository`, `sqlx-repository-macros`)
- [ ] Basic derive macro parsing and code generation
- [ ] PostgreSQL backend implementation
- [ ] Error handling framework
- [ ] Testing infrastructure with testcontainers

#### Phase 1.2: CRUD Operations (Week 2)
- [ ] Create/Insert functionality with proper SQL generation
- [ ] Read operations (find_by_id, find_all)
- [ ] Update operations with optimistic concurrency
- [ ] Delete operations (hard delete foundation)
- [ ] Basic integration tests

#### Phase 1.3: Search & Soft Delete (Week 3)
- [ ] Search parameter parsing and SQL generation
- [ ] Pagination implementation
- [ ] Sorting capabilities
- [ ] Soft delete functionality
- [ ] Comprehensive test coverage

#### Phase 1.4: Polish & Documentation (Week 4)
- [ ] Performance optimization
- [ ] Error message improvements
- [ ] Documentation and examples
- [ ] Benchmarking vs raw sqlx
- [ ] v0.1.0 release preparation

## Considered Alternatives

### 1. Active Record Pattern
**Rejected**: Couples database logic with domain models, harder to test and reason about.

### 2. Query Builder Approach
**Rejected**: More flexible but loses type safety benefits and increases complexity.

### 3. Proc Macro Functions Instead of Derive
**Rejected**: Derive macros provide better ergonomics and clearer intent.

### 4. Runtime Repository Configuration
**Rejected**: Compile-time configuration provides better performance and error detection.

## Consequences

### Positive
- **Type Safety**: Compile-time guarantees for database operations
- **Performance**: Code generation ensures minimal runtime overhead
- **Developer Experience**: Significantly reduced boilerplate for common operations
- **Extensibility**: Foundation supports planned features (audit logging, relationships)
- **Testing**: Generated code is predictable and testable

### Negative
- **Compile Time**: Derive macros increase compilation time
- **Debugging**: Generated code can be harder to debug
- **Learning Curve**: Developers need to understand attribute configuration
- **Flexibility Trade-offs**: Some edge cases may require custom implementations

### Risks & Mitigations
- **Risk**: Macro complexity becomes unmanageable
  - **Mitigation**: Keep macro logic simple, comprehensive tests for edge cases
- **Risk**: Performance regressions vs raw sqlx
  - **Mitigation**: Continuous benchmarking, performance budgets
- **Risk**: PostgreSQL-specific assumptions break multi-database support
  - **Mitigation**: Design backend abstraction from day one

## Success Criteria

### Technical Metrics
- [ ] **Performance**: Within 5% of raw sqlx performance for basic operations
- [ ] **Type Safety**: 100% compile-time validation of basic schema mismatches
- [ ] **Test Coverage**: >95% test coverage including integration tests
- [ ] **Memory Usage**: No significant memory overhead vs raw sqlx

### Functional Requirements
- [ ] **CRUD Operations**: All basic operations working reliably
- [ ] **Search & Pagination**: Flexible search with proper pagination
- [ ] **Soft Delete**: Configurable soft delete with proper query filtering
- [ ] **Error Handling**: Clear, actionable error messages

### Developer Experience
- [ ] **Setup Time**: Repository setup takes <5 minutes for new developers
- [ ] **Learning Curve**: Basic usage learnable in <1 hour with documentation
- [ ] **Debugging**: Clear error messages for common configuration mistakes

## Implementation Plan

### Crate Structure
```
sqlx-repository/
├── crates/
│   ├── sqlx-repository/          # Core library
│   ├── sqlx-repository-macros/   # Derive macros
│   └── sqlx-repository-cli/      # CLI tool (stub for Phase 1)
├── examples/                     # Working examples
├── docs/adr/                     # Architecture decisions
└── tests/                        # Integration tests
```

### Key Files & Responsibilities

#### `sqlx-repository/src/lib.rs`
- Public API exports
- Repository trait definitions
- Core types (SearchParams, SearchResult, etc.)

#### `sqlx-repository/src/backends/postgres.rs`
- PostgreSQL-specific SQL generation
- Database connection handling
- Query execution logic

#### `sqlx-repository-macros/src/lib.rs`
- Derive macro implementation
- Attribute parsing (table, primary_key, soft_delete)
- Code generation for repository implementations

### Testing Strategy
- **Unit Tests**: Individual component testing
- **Integration Tests**: Full database round-trip testing with testcontainers
- **Macro Tests**: Compile-time and runtime macro behavior
- **Performance Tests**: Benchmarks vs raw sqlx operations

### Documentation Requirements
- **Getting Started Guide**: 15-minute tutorial for basic usage
- **API Documentation**: Complete rustdoc coverage
- **Examples**: At least 3 working examples of increasing complexity
- **Migration Guide**: For users coming from raw sqlx

## Future Considerations

This Phase 1 architecture intentionally provides foundation for:
- **Multi-database support** (MySQL, SQLite) via backend abstraction
- **Migration generation** from repository structs
- **Audit logging** through repository method instrumentation
- **Relationship mapping** with lazy loading
- **Validation framework** integrated with repository operations

The backend abstraction and repository trait design ensure these features can be added without breaking changes to the core API.

## Approval

This ADR requires approval from the core team before implementation begins. Implementation will follow the milestone breakdown outlined in ROADMAP.md with regular progress updates and community feedback integration.