//! Database backend implementations
//!
//! This module contains database-specific implementations for different
//! database systems. Currently only PostgreSQL is implemented, with
//! MySQL and SQLite backends planned for future phases.

#[cfg(feature = "postgres")]
pub mod postgres;

// Future database backends
// #[cfg(feature = "mysql")]
// pub mod mysql;

// #[cfg(feature = "sqlite")]
// pub mod sqlite;