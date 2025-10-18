#!/bin/bash

# Build script for Sparrow 2022 project
# This script builds the Rust WASM library and the client

set -e

echo "🔧 Building Rust WebAssembly library..."
cd libs/simulation-wasm
wasm-pack build --target bundler --out-dir pkg
echo "✅ WASM library built successfully"

echo "📦 Installing client dependencies..."
cd ../../client
npm install

echo "🏗️  Building client..."
NODE_OPTIONS="--openssl-legacy-provider" npm run build
echo "✅ Client built successfully"

echo "🎉 Build complete! You can now:"
echo "   - Run 'npm start' in the client directory to start the dev server"
echo "   - Or serve the 'client/dist' directory with any static file server"
