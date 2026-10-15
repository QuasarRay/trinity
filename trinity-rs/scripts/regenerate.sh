#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."
doxygen Doxyfile.trinity-rs
cargo run -p trinity-codegen -- generate
