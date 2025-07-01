//! Unit tests for error handling

use sqlx_repository::{RepositoryError, RepositoryResult};

#[test]
fn test_repository_error_not_found() {
    let error = RepositoryError::not_found("User", "id", 123);
    
    match &error {
        RepositoryError::NotFound { entity, field, value } => {
            assert_eq!(entity, "User");
            assert_eq!(field, "id");
            assert_eq!(value, "123");
        }
        _ => panic!("Expected NotFound error"),
    }
    
    assert_eq!(error.to_string(), "Not found: User with id = 123");
}

#[test]
fn test_repository_error_validation() {
    let error = RepositoryError::validation("Email is invalid");
    
    match &error {
        RepositoryError::Validation(msg) => {
            assert_eq!(msg, "Email is invalid");
        }
        _ => panic!("Expected Validation error"),
    }
    
    assert_eq!(error.to_string(), "Validation error: Email is invalid");
}

#[test]
fn test_repository_error_conflict() {
    let error = RepositoryError::conflict("Email already exists");
    
    match &error {
        RepositoryError::Conflict(msg) => {
            assert_eq!(msg, "Email already exists");
        }
        _ => panic!("Expected Conflict error"),
    }
    
    assert_eq!(error.to_string(), "Conflict: Email already exists");
}

#[test]
fn test_repository_error_configuration() {
    let error = RepositoryError::configuration("Database URL not provided");
    
    match &error {
        RepositoryError::Configuration(msg) => {
            assert_eq!(msg, "Database URL not provided");
        }
        _ => panic!("Expected Configuration error"),
    }
    
    assert_eq!(error.to_string(), "Configuration error: Database URL not provided");
}

#[test]
fn test_repository_error_unsupported_feature() {
    let error = RepositoryError::unsupported_feature("full_text_search", "mysql");
    
    match &error {
        RepositoryError::UnsupportedFeature { feature, backend } => {
            assert_eq!(feature, "full_text_search");
            assert_eq!(backend, "mysql");
        }
        _ => panic!("Expected UnsupportedFeature error"),
    }
    
    assert_eq!(error.to_string(), "Feature 'full_text_search' not supported by mysql backend");
}

#[test]
fn test_repository_result_type_alias() {
    // Test that RepositoryResult is properly aliased
    let success: RepositoryResult<String> = Ok("test".to_string());
    assert!(success.is_ok());
    assert_eq!(success.unwrap(), "test");
    
    let failure: RepositoryResult<String> = Err(RepositoryError::validation("test error"));
    assert!(failure.is_err());
}