# Build stage
FROM rust:1.82-alpine AS builder

# Install build dependencies
RUN apk add --no-cache musl-dev openssl-dev

# Create app directory
WORKDIR /app

# Copy manifests
COPY Cargo.toml ./

# Build dependencies (this is cached as long as Cargo.toml doesn't change)
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm -rf src

# Copy source code
COPY . .

# Build application
RUN touch src/main.rs && cargo build --release

# Runtime stage
FROM alpine:3.19

# Install runtime dependencies
RUN apk add --no-cache libgcc

# Create non-root user
RUN adduser -D -s /bin/sh appuser

# Copy the binary from builder
COPY --from=builder /app/target/release/backend /usr/local/bin/backend

# Copy migrations
COPY migrations /app/migrations

# Change to non-root user
USER appuser

# Set working directory
WORKDIR /app

# Expose port
EXPOSE 3000

# Run the binary
CMD ["backend"]