# PromptParse Rust

.PHONY: build test check format lint clean doc example bench coverage audit

build: ## Build the project
	cargo build

build-release: ## Build the project in release mode
	cargo build --release

test: ## Run all tests
	cargo test

test-unit: ## Run unit tests only
	cargo test --lib

test-integration: ## Run integration tests only
	cargo test --test integration_tests

test-lib: ## Run library tests only
	cargo test --test lib_tests

test-verbose: ## Run tests with verbose output
	cargo test -- --nocapture

check: ## Run cargo check
	cargo check

format: ## Format code with rustfmt
	cargo fmt

format-check: ## Check if code is properly formatted
	cargo fmt --all -- --check

lint: ## Run clippy linter
	cargo clippy --all-targets --all-features

lint-fix: ## Run clippy with automatic fixes
	cargo clippy --all-targets --all-features --fix

clean: ## Clean build artifacts
	cargo clean

doc: ## Generate documentation
	cargo doc --open

doc-private: ## Generate documentation including private items
	cargo doc --document-private-items --open

example: ## Run the basic usage example
	cargo run --example basic_usage

bench: ## Run benchmarks
	cargo test --bench benchmarks --release bench_ -- --nocapture

# Coverage requires cargo-tarpaulin: cargo install cargo-tarpaulin
coverage: ## Generate test coverage report
	cargo tarpaulin --out html --output-dir coverage

coverage-lcov: ## Generate test coverage in lcov format
	cargo tarpaulin --out lcov --output-dir coverage

# Audit requires cargo-audit: cargo install cargo-audit
audit: ## Run security audit
	cargo audit

# Quality checks
quality: format-check lint test ## Run all quality checks

# Package management
package: ## Create a package
	cargo package

publish-dry: ## Dry run of publishing to crates.io
	cargo publish --dry-run

publish: ## Publish to crates.io (use with caution!)
	cargo publish

# Cross-compilation targets (requires targets to be installed)
build-all: ## Build for multiple targets
	cargo build --target x86_64-unknown-linux-gnu
	cargo build --target x86_64-pc-windows-gnu
	cargo build --target x86_64-apple-darwin