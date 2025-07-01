//! Search types and functionality for repository queries.
//!
//! This module provides flexible search capabilities including pagination,
//! filtering, and sorting for repository queries.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Parameters for searching and filtering repository queries
#[derive(Debug, Clone, Deserialize)]
pub struct SearchParams {
    /// Text query for full-text search across searchable fields
    pub query: Option<String>,
    /// Field-specific filters (field_name -> value)
    pub filters: HashMap<String, String>,
    /// Page number for pagination (0-based)
    pub page: u32,
    /// Number of items per page
    pub per_page: u32,
    /// Field to sort by (defaults to "id")
    pub sort_by: Option<String>,
    /// Sort order (ascending or descending)
    pub sort_order: SortOrder,
    /// Record scope for soft delete handling
    pub scope: RecordScope,
}

impl Default for SearchParams {
    fn default() -> Self {
        Self {
            query: None,
            filters: HashMap::new(),
            page: 0,
            per_page: 10,
            sort_by: None,
            sort_order: SortOrder::default(),
            scope: RecordScope::default(),
        }
    }
}

/// Sort order for query results
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum SortOrder {
    /// Ascending order (A-Z, 1-9)
    Asc,
    /// Descending order (Z-A, 9-1)
    Desc,
}

impl Default for SortOrder {
    fn default() -> Self {
        SortOrder::Asc
    }
}

/// Record scope for handling soft-deleted records
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum RecordScope {
    /// Only active (non-deleted) records
    Active,
    /// Only soft-deleted records
    Deleted,
    /// All records (active and deleted)
    All,
}

impl Default for RecordScope {
    fn default() -> Self {
        RecordScope::Active
    }
}

/// Result container for paginated search queries
#[derive(Debug, Serialize)]
pub struct SearchResult<T> {
    /// The actual data items for this page
    pub items: Vec<T>,
    /// Total number of items matching the query (across all pages)
    pub total_count: i64,
    /// Current page number (0-based)
    pub page: u32,
    /// Number of items per page
    pub per_page: u32,
    /// Total number of pages
    pub total_pages: u32,
}

impl<T> SearchResult<T> {
    /// Create a new SearchResult
    pub fn new(items: Vec<T>, total_count: i64, page: u32, per_page: u32) -> Self {
        let total_pages = if per_page > 0 {
            ((total_count as f64) / (per_page as f64)).ceil() as u32
        } else {
            0
        };

        Self {
            items,
            total_count,
            page,
            per_page,
            total_pages,
        }
    }

    /// Check if there are more pages after the current one
    pub fn has_next_page(&self) -> bool {
        self.page + 1 < self.total_pages
    }

    /// Check if there are pages before the current one
    pub fn has_previous_page(&self) -> bool {
        self.page > 0
    }
}