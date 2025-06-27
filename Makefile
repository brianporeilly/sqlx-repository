# Makefile for sqlx-repository testing and development

.PHONY: test test-unit test-integration test-all bench docker-up docker-down clean help

# Default target
help:
	@echo "Available targets:"
	@echo "  test-unit        - Run unit tests (fast, no external dependencies)"
	@echo "  test-integration - Run integration tests (requires Docker)"
	@echo "  test-all         - Run all tests"
	@echo "  test-features    - Test all feature combinations"
	@echo "  bench            - Run benchmarks"
	@echo "  docker-up        - Start test databases"
	@echo "  docker-down      - Stop test databases"
	@echo "  clean            - Clean build artifacts"

# Start test databases
docker-up:
	@echo "Starting test databases..."
	docker-compose -f docker-compose.test.yml up -d
	@echo "Waiting for databases to be ready..."
	sleep 15
	@echo "Databases ready!"

# Stop test databases  
docker-down:
	@echo "Stopping test databases..."
	docker-compose -f docker-compose.test.yml down
	@echo "Databases stopped."

# Run unit tests only (fast, no dependencies)
test-unit:
	@echo "Running unit tests..."
	cargo test --test unit_tests

# Run macro tests
test-macros:
	@echo "Running macro compile tests..."
	cargo test --test macro_tests

# Run integration tests (requires docker-up first)
test-integration: 
	@echo "Running integration tests..."
	@echo "Note: This requires 'make docker-up' to be run first"
	DATABASE_URL_POSTGRES="postgres://test_user:test_pass@localhost:5432/test_db" \
	DATABASE_URL_MYSQL="mysql://test_user:test_pass@localhost:3306/test_db" \
	cargo test --test integration_tests

# Run integration tests with automatic Docker management
test-integration-auto: docker-up
	@echo "Running integration tests with Docker..."
	DATABASE_URL_POSTGRES="postgres://test_user:test_pass@localhost:5432/test_db" \
	DATABASE_URL_MYSQL="mysql://test_user:test_pass@localhost:3306/test_db" \
	cargo test --test integration_tests
	$(MAKE) docker-down

# Run all tests
test-all: test-unit test-macros test-integration-auto

# Run benchmarks
bench:
	@echo "Running benchmarks..."
	cargo bench

# Test with different feature combinations
test-features:
	@echo "Testing feature combinations..."
	@echo "Testing no features..."
	cargo test --no-default-features
	@echo "Testing postgres only..."
	cargo test --no-default-features --features postgres
	@echo "Testing mysql only..."
	cargo test --no-default-features --features mysql  
	@echo "Testing sqlite only..."
	cargo test --no-default-features --features sqlite
	@echo "Testing all features..."
	cargo test --all-features

# Check code formatting and linting
check:
	@echo "Checking code format..."
	cargo fmt -- --check
	@echo "Running clippy..."
	cargo clippy -- -D warnings

# Format code
fmt:
	@echo "Formatting code..."
	cargo fmt

# Clean build artifacts
clean:
	@echo "Cleaning build artifacts..."
	cargo clean

# Generate documentation
docs:
	@echo "Generating documentation..."
	cargo doc --all-features --no-deps --open

# Run quick development checks
dev-check: fmt check test-unit

# Full CI pipeline
ci: fmt check test-features test-all bench