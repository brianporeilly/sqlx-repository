//! Error types for sqlx-repository.
//!
//! This module provides comprehensive error handling for repository operations,
//! including database errors, validation errors, and configuration errors.

use thiserror::Error;

/// Result type alias for repository operations
pub type RepositoryResult<T> = Result<T, RepositoryError>;

/// Error types that can occur during repository operations
#[derive(Error, Debug)]
pub enum RepositoryError {
    /// Database error from sqlx
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    /// Entity not found
    #[error("Not found: {entity} with {field} = {value}")]
    NotFound {
        /// Entity type name
        entity: String,
        /// Field that was searched
        field: String,
        /// Value that was searched for
        value: String,
    },

    /// Validation error
    #[error("Validation error: {0}")]
    Validation(String),

    /// Conflict error (e.g., unique constraint violation)
    #[error("Conflict: {0}")]
    Conflict(String),

    /// Configuration error
    #[error("Configuration error: {0}")]
    Configuration(String),

    /// Feature not supported by current database backend
    #[error("Feature '{feature}' not supported by {backend} backend")]
    UnsupportedFeature {
        /// The unsupported feature
        feature: String,
        /// The backend that doesn't support it
        backend: String,
    },
}

impl RepositoryError {
    /// Create a new NotFound error
    pub fn not_found(entity: &str, field: &str, value: impl std::fmt::Display) -> Self {
        Self::NotFound {
            entity: entity.to_string(),
            field: field.to_string(),
            value: value.to_string(),
        }
    }

    /// Create a new Validation error
    pub fn validation(msg: impl std::fmt::Display) -> Self {
        Self::Validation(msg.to_string())
    }

    /// Create a new Conflict error
    pub fn conflict(msg: impl std::fmt::Display) -> Self {
        Self::Conflict(msg.to_string())
    }

    /// Create a new Configuration error
    pub fn configuration(msg: impl std::fmt::Display) -> Self {
        Self::Configuration(msg.to_string())
    }

    /// Create a new UnsupportedFeature error
    pub fn unsupported_feature(feature: &str, backend: &str) -> Self {
        Self::UnsupportedFeature {
            feature: feature.to_string(),
            backend: backend.to_string(),
        }
    }
}
