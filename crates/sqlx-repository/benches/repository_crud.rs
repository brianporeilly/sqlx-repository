//! Benchmarks for basic CRUD operations

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use sqlx_repository::{RecordScope, SearchParams, SortOrder};
use std::collections::HashMap;

fn bench_search_params_creation(c: &mut Criterion) {
    c.bench_function("search_params_default", |b| {
        b.iter(|| black_box(SearchParams::default()))
    });

    c.bench_function("search_params_with_filters", |b| {
        b.iter(|| {
            let mut filters = HashMap::new();
            filters.insert("status".to_string(), "active".to_string());
            filters.insert("department".to_string(), "engineering".to_string());

            black_box(SearchParams {
                query: Some("test query".to_string()),
                filters,
                page: 1,
                per_page: 20,
                sort_by: Some("name".to_string()),
                sort_order: SortOrder::Desc,
                scope: RecordScope::All,
            })
        })
    });
}

fn bench_postgres_query_building(c: &mut Criterion) {
    use sqlx_repository::backends::postgres::PostgresBackend;

    c.bench_function("postgres_build_select_simple", |b| {
        b.iter(|| {
            black_box(PostgresBackend::build_select_query(
                "users",
                &[],
                &[],
                None,
                None,
            ))
        })
    });

    c.bench_function("postgres_build_select_complex", |b| {
        b.iter(|| {
            black_box(PostgresBackend::build_select_query(
                "users",
                &["id", "name", "email", "status"],
                &["status = $1", "department = $2", "deleted_at IS NULL"],
                Some(20),
                Some(40),
            ))
        })
    });

    c.bench_function("postgres_build_insert", |b| {
        b.iter(|| {
            black_box(PostgresBackend::build_insert_query(
                "users",
                &["name", "email", "status", "department"],
                true,
            ))
        })
    });

    c.bench_function("postgres_build_update", |b| {
        b.iter(|| {
            black_box(PostgresBackend::build_update_query(
                "users",
                &["name", "email", "status"],
                true,
            ))
        })
    });
}

fn bench_error_creation(c: &mut Criterion) {
    use sqlx_repository::RepositoryError;

    c.bench_function("error_not_found", |b| {
        b.iter(|| black_box(RepositoryError::not_found("User", "id", 123)))
    });

    c.bench_function("error_validation", |b| {
        b.iter(|| black_box(RepositoryError::validation("Email is invalid")))
    });

    c.bench_function("error_unsupported_feature", |b| {
        b.iter(|| {
            black_box(RepositoryError::unsupported_feature(
                "full_text_search",
                "mysql",
            ))
        })
    });
}

criterion_group!(
    benches,
    bench_search_params_creation,
    bench_postgres_query_building,
    bench_error_creation
);
criterion_main!(benches);
