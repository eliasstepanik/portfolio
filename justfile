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

# Install required tools
install-tools:
    cargo install trunk
    rustup target add wasm32-unknown-unknown

# Roll back to a stable release tag and rebuild
rollback tag:
    git fetch --tags
    git checkout {{tag}}
    just build
    echo "Rolled back to {{tag}}"