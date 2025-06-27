//! Unit tests for search functionality

use sqlx_repository::{SearchParams, SearchResult, SortOrder, RecordScope};
use std::collections::HashMap;

#[test]
fn test_search_params_default() {
    let params = SearchParams::default();
    
    assert_eq!(params.query, None);
    assert_eq!(params.filters.len(), 0);
    assert_eq!(params.page, 0);
    assert_eq!(params.per_page, 10);
    assert_eq!(params.sort_by, None);
    assert!(matches!(params.sort_order, SortOrder::Asc));
    assert!(matches!(params.scope, RecordScope::Active));
}

#[test]
fn test_search_params_custom() {
    let mut filters = HashMap::new();
    filters.insert("status".to_string(), "active".to_string());
    filters.insert("department".to_string(), "engineering".to_string());
    
    let params = SearchParams {
        query: Some("john".to_string()),
        filters,
        page: 2,
        per_page: 20,
        sort_by: Some("name".to_string()),
        sort_order: SortOrder::Desc,
        scope: RecordScope::All,
    };
    
    assert_eq!(params.query, Some("john".to_string()));
    assert_eq!(params.filters.len(), 2);
    assert_eq!(params.filters.get("status"), Some(&"active".to_string()));
    assert_eq!(params.filters.get("department"), Some(&"engineering".to_string()));
    assert_eq!(params.page, 2);
    assert_eq!(params.per_page, 20);
    assert_eq!(params.sort_by, Some("name".to_string()));
    assert!(matches!(params.sort_order, SortOrder::Desc));
    assert!(matches!(params.scope, RecordScope::All));
}

#[test]
fn test_sort_order_default() {
    let sort_order = SortOrder::default();
    assert!(matches!(sort_order, SortOrder::Asc));
}

#[test]
fn test_record_scope_default() {
    let scope = RecordScope::default();
    assert!(matches!(scope, RecordScope::Active));
}

#[test]
fn test_search_result_new() {
    let items = vec!["item1".to_string(), "item2".to_string()];
    let result = SearchResult::new(items.clone(), 25, 1, 10);
    
    assert_eq!(result.items, items);
    assert_eq!(result.total_count, 25);
    assert_eq!(result.page, 1);
    assert_eq!(result.per_page, 10);
    assert_eq!(result.total_pages, 3); // ceil(25/10) = 3
}

#[test]
fn test_search_result_total_pages_calculation() {
    // Test exact division
    let result = SearchResult::new(vec!["item"], 20, 0, 10);
    assert_eq!(result.total_pages, 2);
    
    // Test remainder
    let result = SearchResult::new(vec!["item"], 21, 0, 10);
    assert_eq!(result.total_pages, 3);
    
    // Test zero per_page
    let result = SearchResult::new(vec!["item"], 10, 0, 0);
    assert_eq!(result.total_pages, 0);
    
    // Test zero total_count
    let result = SearchResult::new(Vec::<String>::new(), 0, 0, 10);
    assert_eq!(result.total_pages, 0);
}

#[test]
fn test_search_result_has_next_page() {
    // Has next page
    let result = SearchResult::new(vec!["item"], 25, 1, 10); // page 1 of 3
    assert!(result.has_next_page());
    
    // Last page
    let result = SearchResult::new(vec!["item"], 25, 2, 10); // page 2 of 3 (0-indexed)
    assert!(!result.has_next_page());
    
    // Single page
    let result = SearchResult::new(vec!["item"], 5, 0, 10); // page 0 of 1
    assert!(!result.has_next_page());
}

#[test]
fn test_search_result_has_previous_page() {
    // First page
    let result = SearchResult::new(vec!["item"], 25, 0, 10);
    assert!(!result.has_previous_page());
    
    // Second page
    let result = SearchResult::new(vec!["item"], 25, 1, 10);
    assert!(result.has_previous_page());
    
    // Last page
    let result = SearchResult::new(vec!["item"], 25, 2, 10);
    assert!(result.has_previous_page());
}

#[test]
fn test_search_params_serde_deserialization() {
    // Test that SearchParams can be deserialized from JSON
    // This is important for web API integration
    let json = r#"{
        "query": "test query",
        "filters": {"status": "active"},
        "page": 1,
        "per_page": 20,
        "sort_by": "name",
        "sort_order": "desc",
        "scope": "all"
    }"#;
    
    let params: SearchParams = serde_json::from_str(json).unwrap();
    
    assert_eq!(params.query, Some("test query".to_string()));
    assert_eq!(params.filters.get("status"), Some(&"active".to_string()));
    assert_eq!(params.page, 1);
    assert_eq!(params.per_page, 20);
    assert_eq!(params.sort_by, Some("name".to_string()));
    assert!(matches!(params.sort_order, SortOrder::Desc));
    assert!(matches!(params.scope, RecordScope::All));
}

#[test]
fn test_search_result_serde_serialization() {
    // Test that SearchResult can be serialized to JSON
    // This is important for web API responses
    let items = vec!["item1".to_string(), "item2".to_string()];
    let result = SearchResult::new(items, 25, 1, 10);
    
    let json = serde_json::to_string(&result).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
    
    assert_eq!(parsed["total_count"], 25);
    assert_eq!(parsed["page"], 1);
    assert_eq!(parsed["per_page"], 10);
    assert_eq!(parsed["total_pages"], 3);
    assert_eq!(parsed["items"][0], "item1");
    assert_eq!(parsed["items"][1], "item2");
}