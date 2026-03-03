#!/bin/bash
# Build WASM package for web frontend

cd "$(dirname "$0")/.." && cd wasm && wasm-pack build --target web --out-dir pkg && cp ../README.md pkg/
