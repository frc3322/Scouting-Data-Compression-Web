#!/bin/bash
# Usage: ./scripts/encode.sh input.csv [output.png] [packed.packed] [--schema schema.json] [--palette palette.json]

cargo run --bin encode -- "$@"

