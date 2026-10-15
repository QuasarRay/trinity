#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."
cargo run -p trinity-codegen -- generate
cargo kani --tests -p trinityal
cargo kani --tests -p carbon-trinity-rs
