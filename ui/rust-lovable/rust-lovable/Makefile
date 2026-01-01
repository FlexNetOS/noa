# Rust Lovable Makefile

.PHONY: help build run test clean install dev web desktop mobile docker deploy

# Default target
help:
	@echo "Rust Lovable - Available commands:"
	@echo "  make build     - Build the project"
	@echo "  make run       - Run the application"
	@echo "  make dev       - Run in development mode"
	@echo "  make web       - Build for web"
	@echo "  make desktop   - Build for desktop"
	@echo "  make mobile    - Build for mobile"
	@echo "  make test      - Run tests"
	@echo "  make clean     - Clean build artifacts"
	@echo "  make install   - Install dependencies"
	@echo "  make docker    - Build Docker image"
	@echo "  make deploy    - Deploy to production"

# Build commands
build:
	cargo build --release

dev:
	cargo run --features web

run:
	cargo run

web:
	cargo build --release --features web

desktop:
	cargo build --release --features desktop

mobile:
	cargo build --release --features mobile

# Testing
test:
	cargo test
test-verbose:
	cargo test -- --nocapture

# Development
dev-web:
	cargo run --features web,hot-reload

dev-desktop:
	cargo run --features desktop,hot-reload

# Dependencies
install:
	rustup update
	cargo install dioxus-cli --locked
	cargo install cargo-watch
	cargo install cargo-audit

check-deps:
	cargo audit

# Code quality
fmt:
	cargo fmt

fmt-check:
	cargo fmt -- --check

clippy:
	cargo clippy -- -D warnings

lint: fmt-check clippy

# Documentation
docs:
	cargo doc --open

# Cleaning
clean:
	cargo clean
	rm -rf dist/
	rm -rf target/

# Docker
docker-build:
	docker build -t rust-lovable:latest .

docker-run:
	docker run -p 8080:8080 rust-lovable:latest

# Environment
env-setup:
	@if [ ! -f .env ]; then cp .env.example .env; echo "Created .env file"; fi

# Database
db-migrate:
	cargo run --bin migrate

db-reset:
	rm -f data/rust-lovable.db
	cargo run --bin migrate

# Performance
bench:
	cargo bench

profile:
	cargo build --release
	perf record ./target/release/rust-lovable
	perf report

# Release
release-check: lint test
	@echo "All checks passed! Ready for release."

release: release-check build
	@echo "Release build completed successfully!"

# Deployment
deploy-web:
	@echo "Deploying to web..."
	# Add your deployment commands here

deploy-desktop:
	@echo "Building desktop packages..."
	# Add desktop packaging commands here

deploy-mobile:
	@echo "Building mobile packages..."
	# Add mobile packaging commands here

deploy: build
	@echo "Deploying to all platforms..."
	$(MAKE) deploy-web
	$(MAKE) deploy-desktop
	$(MAKE) deploy-mobile

# Monitoring
monitor:
	cargo install cargo-tarpaulin
	cargo tarpaulin --out Html

# Development helpers
watch:
	cargo watch -x check -x test -x run

fix:
	cargo fix --allow-dirty --allow-staged

expand:
	cargo expand

# AI Integration testing
test-ai:
	cargo test --test ai_integration_tests

# Load testing
load-test:
	cargo install cargo-criterion
	cargo criterion

# Security
security-check:
	cargo install cargo-audit
	cargo audit
	cargo install cargo-geiger
	cargo geiger

# All-in-one development setup
dev-setup: install env-setup
	@echo "Development environment setup complete!"
	@echo "Run 'make dev' to start development server"

# CI/CD helpers
ci-build: lint test build
	@echo "CI build completed successfully!"

# Feature flags
feature-ai:
	cargo build --features ai-integrations

feature-db:
	cargo build --features database

feature-full:
	cargo build --features full

# Help for specific features
help-features:
	@echo "Available features:"
	@echo "  web          - Web platform support"
	@echo "  desktop      - Desktop platform support"
	@echo "  mobile       - Mobile platform support"
	@echo "  ai-integrations - AI provider integrations"
	@echo "  database     - Database support"
	@echo "  full         - All features enabled"