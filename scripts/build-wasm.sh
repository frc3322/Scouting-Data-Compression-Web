#!/bin/bash
# Build WASM package for web frontend

cd wasm && wasm-pack build --target web --out-dir pkg

