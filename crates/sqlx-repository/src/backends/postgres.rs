//! PostgreSQL-specific implementations
//!
//! This module contains PostgreSQL-specific functionality that will be
//! used by the repository implementations. In Phase 2, this will be
//! refactored to implement a common DatabaseBackend trait.

/// PostgreSQL-specific query helpers
pub struct PostgresBackend;

impl PostgresBackend {
    /// Create a PostgreSQL-specific placeholder for parameter binding
    pub fn placeholder(index: usize) -> String {
        format!("${}", index)
    }

    /// Convert Rust types to PostgreSQL types
    pub fn convert_type(rust_type: &str) -> &str {
        match rust_type {
            "i32" => "INTEGER",
            "i64" => "BIGINT",
            "String" => "VARCHAR",
            "bool" => "BOOLEAN",
            "DateTime<Utc>" => "TIMESTAMP WITH TIME ZONE",
            "NaiveDateTime" => "TIMESTAMP",
            "NaiveDate" => "DATE",
            "NaiveTime" => "TIME",
            "Decimal" => "DECIMAL",
            "f32" => "REAL",
            "f64" => "DOUBLE PRECISION",
            _ => "VARCHAR", // Safe default
        }
    }

    /// Build a SELECT query with PostgreSQL-specific syntax
    pub fn build_select_query(
        table: &str,
        columns: &[&str],
        conditions: &[&str],
        limit: Option<u32>,
        offset: Option<u32>,
    ) -> String {
        let columns_str = if columns.is_empty() {
            "*"
        } else {
            &columns.join(", ")
        };

        let mut query = format!("SELECT {} FROM {}", columns_str, table);

        if !conditions.is_empty() {
            query.push_str(" WHERE ");
            query.push_str(&conditions.join(" AND "));
        }

        if let Some(limit) = limit {
            query.push_str(&format!(" LIMIT {}", limit));
        }

        if let Some(offset) = offset {
            query.push_str(&format!(" OFFSET {}", offset));
        }

        query
    }

    /// Build an INSERT query with PostgreSQL-specific syntax
    pub fn build_insert_query(table: &str, columns: &[&str], returning: bool) -> String {
        let placeholders: Vec<String> = (1..=columns.len()).map(|i| format!("${}", i)).collect();

        let mut query = format!(
            "INSERT INTO {} ({}) VALUES ({})",
            table,
            columns.join(", "),
            placeholders.join(", ")
        );

        if returning {
            query.push_str(" RETURNING *");
        }

        query
    }

    /// Build an UPDATE query with PostgreSQL-specific syntax
    pub fn build_update_query(table: &str, columns: &[&str], returning: bool) -> String {
        let set_clauses: Vec<String> = columns
            .iter()
            .enumerate()
            .map(|(i, col)| format!("{} = ${}", col, i + 1))
            .collect();

        let mut query = format!("UPDATE {} SET {}", table, set_clauses.join(", "));

        // Add WHERE clause placeholder (will be filled by caller)
        query.push_str(&format!(" WHERE id = ${}", columns.len() + 1));

        if returning {
            query.push_str(" RETURNING *");
        }

        query
    }

    /// Build a DELETE query with PostgreSQL-specific syntax
    pub fn build_delete_query(table: &str, soft_delete: bool) -> String {
        if soft_delete {
            format!(
                "UPDATE {} SET deleted_at = NOW(), updated_at = NOW() WHERE id = $1 AND deleted_at IS NULL",
                table
            )
        } else {
            format!("DELETE FROM {} WHERE id = $1", table)
        }
    }
}
