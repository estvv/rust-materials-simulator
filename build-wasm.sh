#!/bin/bash
set -e

echo "==================================="
echo "Building Dynamic Materials for WASM"
echo "==================================="

cargo clean 2>/dev/null || true

export EMCC_CFLAGS="-O3 -sUSE_GLFW=3 -sWASM=1 -sASYNCIFY -sGL_ENABLE_GET_PROC_ADDRESS=1 -sFORCE_FILESYSTEM=1 -sALLOW_MEMORY_GROWTH=1"

echo "Compiling Rust to WebAssembly..."
cargo build --target wasm32-unknown-emscripten --release

OUT_DIR="target/wasm32-unknown-emscripten/release"
WEB_DIR="web"

mkdir -p "$WEB_DIR"

echo "Copying WASM files..."
cp "$OUT_DIR/dynamic_materials.js" "$WEB_DIR/"
cp "$OUT_DIR/dynamic_materials.wasm" "$WEB_DIR/"

echo ""
echo "✓ Build complete!"
echo ""
echo "Files in $WEB_DIR/:"
ls -lh "$WEB_DIR"
echo ""
echo "Next steps:"
echo "  1. Test locally: cd web && python3 -m http.server 8080"
echo "  2. Open browser: http://localhost:8080"
echo ""
