#!/bin/bash
set -e

echo "==================================="
echo "Building Dynamic Materials for WASM"
echo "==================================="

# Build with Emscripten (preload config files)
export EMCC_CFLAGS="-O3 -sUSE_GLFW=3 -sASSERTIONS=0 -sWASM=1 -sGL_ENABLE_GET_PROC_ADDRESS=1 -sFORCE_FILESYSTEM=1 -sALLOW_MEMORY_GROWTH=1 --preload-file ./config@config"

echo "Compiling Rust to WebAssembly..."
cargo build --target wasm32-unknown-emscripten --release

OUT_DIR="target/wasm32-unknown-emscripten/release"
WEB_DIR="web"

mkdir -p "$WEB_DIR"

echo "Copying WASM files..."
cp "$OUT_DIR/dynamic_materials.js" "$WEB_DIR/"
cp "$OUT_DIR/dynamic_materials.wasm" "$WEB_DIR/"

# Copy preload data file if it exists
if [ -f "$OUT_DIR/dynamic_materials.data" ]; then
    cp "$OUT_DIR/dynamic_materials.data" "$WEB_DIR/"
    echo "✓ Copied config data bundle"
fi

echo ""
echo "✓ Build complete!"
echo ""
echo "Files in $WEB_DIR/:"
ls -lh "$WEB_DIR"