# Multi-stage build for Rust Lovable
FROM rust:1.85.0 AS builder

# Install system dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    build-essential \
    && rm -rf /var/lib/apt/lists/*

# Set working directory
WORKDIR /app

# Copy manifests + build script + local path dependencies.
# Cargo needs local path dependency manifests to exist even when the deps are optional.
COPY Cargo.toml Cargo.lock build.rs ./
COPY crates ./crates

# Build dependencies (this is cached)
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release --no-default-features --features server
RUN rm -rf src

# Copy source code
COPY src ./src
COPY assets ./assets
COPY templates ./templates

# Build the application
RUN cargo build --release --no-default-features --features server

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Create app user
RUN useradd -m -u 1000 appuser

# Set working directory
WORKDIR /app

# Copy binary from builder
COPY --from=builder /app/target/release/rust-lovable /app/rust-lovable

# Copy assets
COPY --from=builder /app/assets /app/assets
COPY --from=builder /app/templates /app/templates

# Create data directory
RUN mkdir -p /app/data && chown -R appuser:appuser /app

# Switch to non-root user
USER appuser

# Expose port
EXPOSE 8080

# Bind on all interfaces inside the container
ENV RUST_LOVABLE_ADDRESS=0.0.0.0:8080

# Health check
# Basic health check (server root should respond)
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:8080/ || exit 1

# Run the application
CMD ["./rust-lovable"]