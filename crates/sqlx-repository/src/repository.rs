//! Core repository trait and implementations

use crate::error::{RepositoryError, RepositoryResult};
use crate::search::{RecordScope, SearchParams, SearchResult, SortOrder};
use async_trait::async_trait;
use sqlx::FromRow;

/// Core repository trait providing CRUD operations and search functionality
#[async_trait]
pub trait Repository<T>: Send + Sync
where
    T: for<'r> FromRow<'r, sqlx::postgres::PgRow> + Send + Unpin,
{
    /// Type for creating new entities
    type CreateType: Send;
    /// Type for updating existing entities  
    type UpdateType: Send;

    // Required implementations (provided by derive macro)
    /// Get the database table name for this repository
    fn table_name() -> &'static str;
    /// Check if soft delete is enabled for this repository
    fn soft_delete_enabled() -> bool {
        false
    }
    /// Get list of fields that can be searched with text queries
    fn searchable_fields() -> &'static [&'static str] {
        &[]
    }
    /// Get list of fields that can be filtered
    fn filterable_fields() -> &'static [&'static str] {
        &[]
    }
    /// Get the database connection pool
    fn pool(&self) -> &sqlx::PgPool;

    // Required method implementations (must be provided by implementor)
    /// Create a new entity
    async fn create(&self, data: Self::CreateType) -> RepositoryResult<T>;
    /// Update an existing entity by ID
    async fn update(&self, id: i32, data: Self::UpdateType) -> RepositoryResult<Option<T>>;

    // Default implementations using PostgreSQL (will be abstracted in Phase 2)
    /// Find an entity by its ID
    async fn find_by_id(&self, id: i32) -> RepositoryResult<Option<T>> {
        let query = if Self::soft_delete_enabled() {
            format!(
                "SELECT * FROM {} WHERE id = $1 AND deleted_at IS NULL",
                Self::table_name()
            )
        } else {
            format!("SELECT * FROM {} WHERE id = $1", Self::table_name())
        };

        sqlx::query_as(&query)
            .bind(id)
            .fetch_optional(self.pool())
            .await
            .map_err(RepositoryError::from)
    }

    /// Find all entities
    async fn find_all(&self) -> RepositoryResult<Vec<T>> {
        let query = if Self::soft_delete_enabled() {
            format!(
                "SELECT * FROM {} WHERE deleted_at IS NULL ORDER BY id",
                Self::table_name()
            )
        } else {
            format!("SELECT * FROM {} ORDER BY id", Self::table_name())
        };

        sqlx::query_as(&query)
            .fetch_all(self.pool())
            .await
            .map_err(RepositoryError::from)
    }

    /// Delete an entity by ID (soft delete if enabled, otherwise hard delete)
    async fn delete(&self, id: i32) -> RepositoryResult<bool> {
        let query = if Self::soft_delete_enabled() {
            format!("UPDATE {} SET deleted_at = NOW(), updated_at = NOW() WHERE id = $1 AND deleted_at IS NULL", Self::table_name())
        } else {
            format!("DELETE FROM {} WHERE id = $1", Self::table_name())
        };

        let result = sqlx::query(&query)
            .bind(id)
            .execute(self.pool())
            .await
            .map_err(RepositoryError::from)?;

        Ok(result.rows_affected() > 0)
    }

    /// Search entities with filtering, pagination, and sorting
    async fn search(&self, params: SearchParams) -> RepositoryResult<SearchResult<T>> {
        let mut conditions = Vec::new();
        let mut bind_count = 0;
        let mut bind_values: Vec<String> = Vec::new();

        // Handle soft delete scope
        if Self::soft_delete_enabled() {
            match params.scope {
                RecordScope::Active => conditions.push("deleted_at IS NULL".to_string()),
                RecordScope::Deleted => conditions.push("deleted_at IS NOT NULL".to_string()),
                RecordScope::All => {} // No condition needed
            }
        }

        // Handle text search
        if let Some(query) = &params.query {
            if !Self::searchable_fields().is_empty() && !query.trim().is_empty() {
                bind_count += 1;
                let search_conditions = Self::searchable_fields()
                    .iter()
                    .map(|field| format!("{} ILIKE ${}", field, bind_count))
                    .collect::<Vec<_>>()
                    .join(" OR ");
                conditions.push(format!("({})", search_conditions));
                bind_values.push(format!("%{}%", query));
            }
        }

        // Handle field filters
        for (field, value) in &params.filters {
            if Self::filterable_fields().contains(&field.as_str()) {
                bind_count += 1;
                conditions.push(format!("{} = ${}", field, bind_count));
                bind_values.push(value.clone());
            }
        }

        let where_clause = if conditions.is_empty() {
            String::new()
        } else {
            format!(" WHERE {}", conditions.join(" AND "))
        };

        // Build count query
        let count_query = format!(
            "SELECT COUNT(*) FROM {}{}",
            Self::table_name(),
            where_clause
        );

        // Build main query with sorting and pagination
        let sort_field = params.sort_by.as_deref().unwrap_or("id");
        let sort_order = match params.sort_order {
            SortOrder::Asc => "ASC",
            SortOrder::Desc => "DESC",
        };
        let offset = params.page * params.per_page;

        let main_query = format!(
            "SELECT * FROM {}{} ORDER BY {} {} LIMIT {} OFFSET {}",
            Self::table_name(),
            where_clause,
            sort_field,
            sort_order,
            params.per_page,
            offset
        );

        // Execute queries
        let mut count_query_builder = sqlx::query_scalar(&count_query);
        let mut main_query_builder = sqlx::query_as(&main_query);

        for value in &bind_values {
            count_query_builder = count_query_builder.bind(value);
            main_query_builder = main_query_builder.bind(value);
        }

        let total_count: i64 = count_query_builder
            .fetch_one(self.pool())
            .await
            .map_err(RepositoryError::from)?;

        let items: Vec<T> = main_query_builder
            .fetch_all(self.pool())
            .await
            .map_err(RepositoryError::from)?;

        Ok(SearchResult::new(
            items,
            total_count,
            params.page,
            params.per_page,
        ))
    }

    /// Restore a soft-deleted entity by ID
    async fn restore(&self, id: i32) -> RepositoryResult<Option<T>>
    where
        Self: Sized,
    {
        if !Self::soft_delete_enabled() {
            return Err(RepositoryError::configuration("Soft delete not enabled"));
        }

        let query = format!(
            "UPDATE {} SET deleted_at = NULL, updated_at = NOW() WHERE id = $1 RETURNING *",
            Self::table_name()
        );

        sqlx::query_as(&query)
            .bind(id)
            .fetch_optional(self.pool())
            .await
            .map_err(RepositoryError::from)
    }

    /// Permanently delete an entity by ID (ignores soft delete setting)
    async fn hard_delete(&self, id: i32) -> RepositoryResult<bool> {
        let result = sqlx::query(&format!("DELETE FROM {} WHERE id = $1", Self::table_name()))
            .bind(id)
            .execute(self.pool())
            .await
            .map_err(RepositoryError::from)?;

        Ok(result.rows_affected() > 0)
    }

    /// Count entities matching the given search parameters
    async fn count(&self, params: SearchParams) -> RepositoryResult<i64> {
        let mut conditions = Vec::new();
        let mut bind_values: Vec<String> = Vec::new();

        // Handle soft delete scope
        if Self::soft_delete_enabled() {
            match params.scope {
                RecordScope::Active => conditions.push("deleted_at IS NULL".to_string()),
                RecordScope::Deleted => conditions.push("deleted_at IS NOT NULL".to_string()),
                RecordScope::All => {} // No condition needed
            }
        }

        // Handle text search
        if let Some(query) = &params.query {
            if !Self::searchable_fields().is_empty() && !query.trim().is_empty() {
                let search_conditions = Self::searchable_fields()
                    .iter()
                    .map(|field| format!("{} ILIKE $1", field))
                    .collect::<Vec<_>>()
                    .join(" OR ");
                conditions.push(format!("({})", search_conditions));
                bind_values.push(format!("%{}%", query));
            }
        }

        // Handle field filters
        let mut bind_count = if bind_values.is_empty() { 0 } else { 1 };
        for (field, value) in &params.filters {
            if Self::filterable_fields().contains(&field.as_str()) {
                bind_count += 1;
                conditions.push(format!("{} = ${}", field, bind_count));
                bind_values.push(value.clone());
            }
        }

        let where_clause = if conditions.is_empty() {
            String::new()
        } else {
            format!(" WHERE {}", conditions.join(" AND "))
        };

        let count_query = format!(
            "SELECT COUNT(*) FROM {}{}",
            Self::table_name(),
            where_clause
        );
        let mut query_builder = sqlx::query_scalar(&count_query);

        for value in &bind_values {
            query_builder = query_builder.bind(value);
        }

        query_builder
            .fetch_one(self.pool())
            .await
            .map_err(RepositoryError::from)
    }
}
