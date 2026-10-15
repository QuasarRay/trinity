# Generated files

Canonical generator: `cargo run -p trinity-codegen -- generate`.

Inputs:

1. original Trinity/Trinity AL tests under `../trinity/tests` and `../trinityal/tests`;
2. Doxygen XML produced by `Doxyfile.trinity-rs`;
3. Rust `#[trinity_test]` declarations under `crates/*/tests`.

Outputs include the compatibility inventory, Doxygen symbol catalogue, test matrix and per-crate Kani integration-test targets. Generated proof wrappers are never semantic placeholders: only a mapped Rust semantic body can receive `#[kani::proof]`.
