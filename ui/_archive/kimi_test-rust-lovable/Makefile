# Rust Lovable Makefile

.PHONY: \
	help build run test clean install dev \
	web desktop mobile \
	dev-web dev-desktop run-server build-server dev-server \
	dev-fullstack-web dev-fullstack-desktop \
	docker docker-build docker-run deploy

# Default target
help:
	@echo "Rust Lovable - Available commands:"
	@echo "  make build     - Build the project"
	@echo "  make run       - Run the application"
	@echo "  make dev       - Run in development mode (desktop hot reload)"
	@echo "  make dev-web   - Dev (web) with hot reload"
	@echo "  make dev-desktop - Dev (desktop) with hot reload"
	@echo "  make dev-server  - Dev (server) with autorestart"
	@echo "  make run-server  - Run server mode"
	@echo "  make web       - Build for web"
	@echo "  make desktop   - Build for desktop"
	@echo "  make mobile    - Build for mobile"
	@echo "  make test      - Run tests"
	@echo "  make clean     - Clean build artifacts"
	@echo "  make install   - Install dependencies"
	@echo "  make docker    - Build Docker image"
	@echo "  make deploy    - Deploy to production"
	@echo ""
	@echo "Fullstack helpers:"
	@echo "  make dev-fullstack-web     - Fullstack (server+web) w/ hot reload"
	@echo "  make dev-fullstack-desktop - Fullstack (server+desktop) w/ hot reload"

# Build commands
build:
	cargo build --release

dev:
	$(MAKE) dev-desktop

run:
	cargo run

build-server:
	cargo build --release --no-default-features --features server

run-server:
	cargo run --no-default-features --features server

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
	dx serve --web --fullstack false --hot-reload true

dev-desktop:
	dx serve --desktop --fullstack false --hot-reload true

# Fullstack development (server + client)
# Note: we pass --no-default-features to avoid pulling in desktop defaults when targeting web/server.
dev-fullstack-web:
	dx serve --web --fullstack true --no-default-features --features "server web" --hot-reload true

dev-fullstack-desktop:
	dx serve --desktop --fullstack true --no-default-features --features "server desktop" --hot-reload true

# Server dev with autorestart (requires cargo-watch)
dev-server:
	cargo watch -x "run --no-default-features --features server"

# Dependencies
install:
	rustup update
	rustup target add wasm32-unknown-unknown
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
	@echo Cleaning extra build outputs...
ifeq ($(OS),Windows_NT)
	@if exist dist rmdir /s /q dist
	@if exist target rmdir /s /q target
else
	rm -rf dist/
	rm -rf target/
endif

# Docker
docker-build:
	docker build -t rust-lovable:latest .

docker-run:
	docker run -p 8080:8080 rust-lovable:latest

# Alias to match the help output
docker: docker-build

# Environment
env-setup:
ifeq ($(OS),Windows_NT)
	@if not exist .env copy .env.example .env >NUL && echo Created .env file
else
	@if [ ! -f .env ]; then cp .env.example .env; echo "Created .env file"; fi
endif

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
	@echo TODO: Add your deployment commands here

deploy-desktop:
	@echo "Building desktop packages..."
	@echo TODO: Add desktop packaging commands here

deploy-mobile:
	@echo "Building mobile packages..."
	@echo TODO: Add mobile packaging commands here

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
	@echo "  server       - Server mode (Axum + Dioxus fullstack server)"
	@echo "  ai-integrations - AI provider integrations"
	@echo "  database     - Database support"
	@echo ""
	@echo "Note: There is no single 'full' feature flag; enable the ones you need."