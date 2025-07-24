# justfile
set dotenv-load          # .env is loaded automatically

preflight:
    cargo fmt --all -- --check
    cargo clippy --workspace --all-targets --all-features -- -D warnings
    cargo test --workspace
    cargo doc --workspace --no-deps --document-private-items

build: preflight           # run depends on a clean preflight
    cargo build --workspace

# Run the web application in development mode
run:
    cd frontend && trunk serve --open

# Development server without opening browser
serve:
    cd frontend && trunk serve

# Build for production
build-web:
    cd frontend && trunk build --release

# Build and serve production build locally
serve-prod: build-web
    cd frontend/dist && python3 -m http.server 8000

# Clean build artifacts
clean:
    cargo clean
    rm -rf frontend/dist

# Run backend in development mode
backend-dev:
    cd backend && cargo run

# Run both frontend and backend
dev:
    docker-compose up -d postgres
    just backend-dev &
    just serve

# Docker compose commands
docker-up:
    docker-compose up -d

docker-down:
    docker-compose down

docker-logs:
    docker-compose logs -f

# Database commands
db-create:
    cd backend && sqlx database create

db-migrate:
    cd backend && sqlx migrate run

db-setup: db-create db-migrate

# Install required tools
install-tools:
    cargo install trunk
    cargo install sqlx-cli --no-default-features --features postgres
    rustup target add wasm32-unknown-unknown

# Roll back to a stable release tag and rebuild
rollback tag:
    git fetch --tags
    git checkout {{tag}}
    just build
    echo "Rolled back to {{tag}}"