//! Unit tests for database backend functionality

use sqlx_repository::backends::postgres::PostgresBackend;

#[test]
fn test_postgres_placeholder() {
    assert_eq!(PostgresBackend::placeholder(1), "$1");
    assert_eq!(PostgresBackend::placeholder(5), "$5");
    assert_eq!(PostgresBackend::placeholder(100), "$100");
}

#[test]
fn test_postgres_convert_type() {
    assert_eq!(PostgresBackend::convert_type("i32"), "INTEGER");
    assert_eq!(PostgresBackend::convert_type("i64"), "BIGINT");
    assert_eq!(PostgresBackend::convert_type("String"), "VARCHAR");
    assert_eq!(PostgresBackend::convert_type("bool"), "BOOLEAN");
    assert_eq!(
        PostgresBackend::convert_type("DateTime<Utc>"),
        "TIMESTAMP WITH TIME ZONE"
    );
    assert_eq!(PostgresBackend::convert_type("NaiveDateTime"), "TIMESTAMP");
    assert_eq!(PostgresBackend::convert_type("NaiveDate"), "DATE");
    assert_eq!(PostgresBackend::convert_type("NaiveTime"), "TIME");
    assert_eq!(PostgresBackend::convert_type("Decimal"), "DECIMAL");
    assert_eq!(PostgresBackend::convert_type("f32"), "REAL");
    assert_eq!(PostgresBackend::convert_type("f64"), "DOUBLE PRECISION");

    // Unknown type should default to VARCHAR
    assert_eq!(PostgresBackend::convert_type("CustomType"), "VARCHAR");
}

#[test]
fn test_postgres_build_select_query() {
    // Basic select all
    let query = PostgresBackend::build_select_query("users", &[], &[], None, None);
    assert_eq!(query, "SELECT * FROM users");

    // Select specific columns
    let query = PostgresBackend::build_select_query("users", &["id", "name"], &[], None, None);
    assert_eq!(query, "SELECT id, name FROM users");

    // With conditions
    let query = PostgresBackend::build_select_query(
        "users",
        &[],
        &["id = $1", "name ILIKE $2"],
        None,
        None,
    );
    assert_eq!(query, "SELECT * FROM users WHERE id = $1 AND name ILIKE $2");

    // With limit
    let query = PostgresBackend::build_select_query("users", &[], &[], Some(10), None);
    assert_eq!(query, "SELECT * FROM users LIMIT 10");

    // With offset
    let query = PostgresBackend::build_select_query("users", &[], &[], None, Some(20));
    assert_eq!(query, "SELECT * FROM users OFFSET 20");

    // Complete query
    let query = PostgresBackend::build_select_query(
        "users",
        &["id", "name"],
        &["status = $1"],
        Some(10),
        Some(20),
    );
    assert_eq!(
        query,
        "SELECT id, name FROM users WHERE status = $1 LIMIT 10 OFFSET 20"
    );
}

#[test]
fn test_postgres_build_insert_query() {
    // Basic insert without returning
    let query = PostgresBackend::build_insert_query("users", &["name", "email"], false);
    assert_eq!(query, "INSERT INTO users (name, email) VALUES ($1, $2)");

    // Insert with returning
    let query = PostgresBackend::build_insert_query("users", &["name", "email"], true);
    assert_eq!(
        query,
        "INSERT INTO users (name, email) VALUES ($1, $2) RETURNING *"
    );

    // Single column
    let query = PostgresBackend::build_insert_query("users", &["name"], true);
    assert_eq!(query, "INSERT INTO users (name) VALUES ($1) RETURNING *");
}

#[test]
fn test_postgres_build_update_query() {
    // Basic update without returning
    let query = PostgresBackend::build_update_query("users", &["name", "email"], false);
    assert_eq!(
        query,
        "UPDATE users SET name = $1, email = $2 WHERE id = $3"
    );

    // Update with returning
    let query = PostgresBackend::build_update_query("users", &["name", "email"], true);
    assert_eq!(
        query,
        "UPDATE users SET name = $1, email = $2 WHERE id = $3 RETURNING *"
    );

    // Single column update
    let query = PostgresBackend::build_update_query("users", &["name"], true);
    assert_eq!(
        query,
        "UPDATE users SET name = $1 WHERE id = $2 RETURNING *"
    );
}

#[test]
fn test_postgres_build_delete_query() {
    // Hard delete
    let query = PostgresBackend::build_delete_query("users", false);
    assert_eq!(query, "DELETE FROM users WHERE id = $1");

    // Soft delete
    let query = PostgresBackend::build_delete_query("users", true);
    assert_eq!(query, "UPDATE users SET deleted_at = NOW(), updated_at = NOW() WHERE id = $1 AND deleted_at IS NULL");
}

#[test]
fn test_postgres_query_building_consistency() {
    // Test that placeholder numbering is consistent
    let select_query =
        PostgresBackend::build_select_query("users", &[], &["name = $1", "email = $2"], None, None);
    assert!(select_query.contains("$1"));
    assert!(select_query.contains("$2"));

    let insert_query = PostgresBackend::build_insert_query("users", &["name", "email"], false);
    assert!(insert_query.contains("$1"));
    assert!(insert_query.contains("$2"));

    let update_query = PostgresBackend::build_update_query("users", &["name", "email"], false);
    assert!(update_query.contains("$1"));
    assert!(update_query.contains("$2"));
    assert!(update_query.contains("$3")); // WHERE id = $3
}
