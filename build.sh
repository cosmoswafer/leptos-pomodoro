#!/bin/bash
# Build script for local development

echo "Building Leptos Pomodoro Timer..."

# Build the WASM target
cargo build --target wasm32-unknown-unknown --release

# Generate WASM bindings
wasm-bindgen --out-dir pkg --target web --no-typescript target/wasm32-unknown-unknown/release/leptos_pomodoro.wasm

# Prepare distribution files
mkdir -p dist
cp index.html dist/
cp -r pkg dist/

echo "Build complete! Files are in the 'dist' directory."
echo "To serve locally, run: cd dist && python3 -m http.server 8080"