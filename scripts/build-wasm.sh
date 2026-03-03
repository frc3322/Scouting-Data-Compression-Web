#!/bin/bash
# Build WASM package for web frontend

set -e
cd "$(dirname "$0")/.."
cd wasm
# Ensure README exists for wasm-pack (symlink or copy)
[[ -L README.md ]] || cp ../README.md README.md
wasm-pack build --target web --out-dir pkg
# Explicit copy to pkg root (npm requires README.md at package root)
cp README.md pkg/README.md
# Ensure README.md is included in npm package (wasm-pack "files" may omit it)
node -e "
const fs = require('fs');
const pkg = JSON.parse(fs.readFileSync('pkg/package.json', 'utf8'));
pkg.files = pkg.files || [];
if (!pkg.files.includes('README.md')) pkg.files.push('README.md');
fs.writeFileSync('pkg/package.json', JSON.stringify(pkg, null, 2));
"
