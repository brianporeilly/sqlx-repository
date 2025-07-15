# Makefile for sqlx-repository
#
# Testing Strategy:
# - `make test`: Fast tests (unit + macro compilation) - no database required
# - `make test-unit`: Unit tests only (23 tests) - business logic validation
# - `make test-macro`: Macro compilation tests (9 tests) - derive macro validation  
# - `make test-lib`: Library embedded tests (8 tests) - utility functions
# - `make test-integration`: Full database integration tests - requires PostgreSQL
# - `make test-all`: All tests including integration - full test suite
#
# Development Commands:
# - `make watch`: Watch for changes and run unit tests
# - `make quick-test`: Alias for test-unit
# - `make dev`: Start development environment with database

.PHONY: help build test test-unit test-macro test-lib test-integration test-integration-keep-db test-all check lint fmt clean docker-up docker-down install-tools deps

# Default target
help: ## Show this help message
	@echo "Available targets:"
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "  \033[36m%-20s\033[0m %s\n", $$1, $$2}'

# Development
build: ## Build all crates
	cargo build --workspace

test: ## Run all tests (unit + macro tests, no database required)
	cargo test --test unit_tests --test macro_tests --workspace

test-unit: ## Run unit tests only
	cargo test --test unit_tests --workspace

test-macro: ## Run macro compilation tests
	cargo test --test macro_tests --workspace

test-lib: ## Run library embedded tests
	cargo test --lib --workspace

test-all: ## Run all tests including integration (requires database)
	$(MAKE) test
	$(MAKE) test-integration

check: ## Run cargo check
	cargo check --workspace

lint: ## Run clippy lints
	cargo clippy --workspace -- -D warnings

fmt: ## Format code
	cargo fmt --all

fmt-check: ## Check code formatting
	cargo fmt --all -- --check

clean: ## Clean build artifacts
	cargo clean

# Testing with database
test-integration: docker-up ## Run integration tests with database
	@echo "Waiting for PostgreSQL to be ready..."
	sleep 5
	cargo test --test integration_tests --workspace -- --test-threads=1
	$(MAKE) docker-down

test-integration-keep-db: docker-up ## Run integration tests but keep database running
	@echo "Waiting for PostgreSQL to be ready..."
	sleep 5
	cargo test --test integration_tests --workspace -- --test-threads=1
	@echo "Database still running. Use 'make docker-down' to stop."

test-coverage: ## Generate test coverage report
	cargo tarpaulin --workspace --out Html --output-dir coverage/

# Dependencies and tools
install-tools: ## Install development tools
	cargo install cargo-tarpaulin cargo-audit cargo-outdated

deps: ## Check dependency status
	cargo outdated
	cargo audit

# Docker
docker-up: ## Start PostgreSQL test database
	docker-compose up -d postgres_test

docker-down: ## Stop PostgreSQL test database
	docker-compose down

docker-logs: ## View database logs
	docker-compose logs -f postgres_test

docker-up-dev: ## Start PostgreSQL development database
	docker-compose up -d postgres

docker-up-both: ## Start both PostgreSQL databases
	docker-compose up -d postgres postgres_test

# Database management
db-create: ## Create test database
	docker-compose exec postgres createdb -U postgres sqlx_repository_test || true

db-drop: ## Drop test database
	docker-compose exec postgres dropdb -U postgres sqlx_repository_test || true

db-reset: db-drop db-create ## Reset test database

# Migration commands
migrate-dev: ## Run migrations against development database
	@echo "Running migrations against development database..."
	cd crates/sqlx-repository && DATABASE_URL="postgres://postgres:password@localhost:5432/sqlx_repository_dev" sqlx migrate run --source migrations

migrate-test: ## Run migrations against test database
	@echo "Running migrations against test database..."
	cd crates/sqlx-repository && DATABASE_URL="postgres://postgres:password@localhost:5433/sqlx_repository_test" sqlx migrate run --source migrations

migrate-info: ## Show migration status
	@echo "Migration status for test database:"
	cd crates/sqlx-repository && DATABASE_URL="postgres://postgres:password@localhost:5433/sqlx_repository_test" sqlx migrate info --source migrations

# CLI development
cli-build: ## Build CLI tool
	cargo build --bin sqlx-repository

cli-install: ## Install CLI tool locally
	cargo install --path crates/sqlx-repository-cli

# Release
release-check: fmt-check lint test ## Run all checks for release
	@echo "All checks passed! Ready for release."

# Documentation
docs: ## Generate documentation
	cargo doc --workspace --open

docs-serve: ## Serve documentation locally
	cargo doc --workspace --no-deps
	@echo "Documentation available at: file://$(PWD)/target/doc/sqlx_repository/index.html"

# Benchmarks
bench: ## Run benchmarks
	cargo bench --workspace

# Examples
run-examples: ## Run all examples
	@for example in examples/*.rs; do \
		echo "Running $$example..."; \
		cargo run --example $$(basename $$example .rs) || true; \
	done

# Development workflow
dev: ## Start development environment
	$(MAKE) docker-up
	@echo "Development environment ready!"
	@echo "- Database: PostgreSQL on localhost:5432"
	@echo "- Run 'make test' to run tests"
	@echo "- Run 'make docker-down' when done"

# CI/CD helpers
ci-test: ## Run tests in CI environment
	cargo test --workspace --verbose

ci-lint: ## Run linting in CI environment
	cargo clippy --workspace --all-targets --all-features -- -D warnings
	cargo fmt --all -- --check

ci-security: ## Run security checks
	cargo audit
	cargo deny check

# Database-specific testing
test-postgres: docker-up ## Test PostgreSQL backend specifically
	sleep 5
	cargo test --workspace --features postgres -- --test-threads=1
	$(MAKE) docker-down

# Quick development commands
quick-test: ## Run fast tests only (unit tests)
	cargo test --test unit_tests --workspace

watch: ## Watch for changes and run fast tests
	cargo watch -x 'test --test unit_tests --workspace'

watch-check: ## Watch for changes and run check
	cargo watch -x 'check --workspace'

watch-test-all: ## Watch for changes and run all tests (no database)
	cargo watch -x 'test --test unit_tests --test macro_tests --workspace'