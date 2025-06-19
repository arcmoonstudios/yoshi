#!/bin/bash
# Build script for WASM components of Yoshi MCP

set -e

echo "Building Yoshi MCP WebAssembly components..."

# Ensure we have the WASM target
rustup target add wasm32-unknown-unknown

# Build with optimizations for WASM
echo "Building WASM module..."
RUSTFLAGS="-C target-feature=+simd128" \
wasm-pack build . \
  --target web \
  --out-dir dist/wasm \
  --release \
  --scope arcmoon-studios

# Optimize WASM binary if wasm-opt is available
if command -v wasm-opt &> /dev/null; then
    echo "Optimizing WASM binary..."
    wasm-opt --enable-simd --enable-bulk-memory -O3 \
      dist/wasm/yoshi_mcp_bg.wasm \
      -o dist/wasm/yoshi_mcp_bg.wasm
else
    echo "wasm-opt not found, skipping optimization"
fi

# Generate TypeScript definitions for bundler target
echo "Generating TypeScript definitions..."
wasm-pack build . \
  --target bundler \
  --out-dir dist/wasm-ts \
  --release \
  --scope arcmoon-studios

echo "WASM build complete!"
echo "Web target: dist/wasm/"
echo "TypeScript target: dist/wasm-ts/"
