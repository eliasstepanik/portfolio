#!/bin/bash
# WASM build script for Leptos without Trunk

# Build the WASM binary
echo "Building WASM..."
cargo build --package frontend --target wasm32-unknown-unknown --release

# Create dist directory
mkdir -p frontend/dist

# Run wasm-bindgen
echo "Running wasm-bindgen..."
~/.cargo/bin/wasm-bindgen --target web \
    --out-dir frontend/dist \
    --out-name frontend \
    target/wasm32-unknown-unknown/release/frontend.wasm

# Copy the index.html file
cp frontend/index.html frontend/dist/

# Copy static assets
cp -r frontend/public/* frontend/dist/

echo "Build complete! Files in frontend/dist/"
echo "You can serve with: cd frontend/dist && python3 -m http.server 8000"