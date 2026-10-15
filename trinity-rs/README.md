# Trinity-rs

- Status: experimental Carbon-compatible reimplementation on `trinity-rs-dev`
- Legacy baseline: `a9343c8904310374f284c38a71998af0672b3184`
- Uploaded machine-readable documentation SHA-256: `9f77e297e1c0bcb83f0cac2cdb390ac43de9e341db9d11e6e0efce9ad59ec255`
- Runtime kernel: Bevy 0.20.0
- Python bridge: PyO3 0.30.1
- Code generation: Doxygen XML + Genco 0.20.0 + `proc_macro2`
- Verification: native tests + generated Kani proof harnesses

`trinity-rs/` is additive. The original C++/Python Trinity and Trinity AL sources remain the compatibility oracle.
The Rust port does not hide Bevy behind a neutral engine abstraction: ECS entities/components, assets,
reflection, plugin ordering, schedules and the enabled Bevy render stack are the implementation substrate.
Carbon names remain the public compatibility names.

## Workspace

- `vendor/blue-rs/`: vendored Blue-rs/Blueexposure overlay used by this branch until it is published as a reusable crate.
- `crates/trinity-meta/`: Carbon/Blue/Bevy metadata and registration bridge.
- `crates/trinity-macros/`: derive and attribute macros. Macro implementation uses `proc_macro2`; Rust still requires a separate proc-macro crate for consuming those macros.
- `crates/trinityal/`: Trinity AL value/resource abstractions implemented with Bevy assets/reflection.
- `crates/trinity/`: Trinity ECS/runtime/plugin implementation.
- `crates/trinity-py/`: PyO3 development module and Carbon-name bridge.
- `crates/trinity-codegen/`: Doxygen/source/test discovery and Genco emission.
- `generated/`: checked/generated compatibility inventories and Kani targets.
- `architecture/ISO-IEC-IEEE-42010-2022.md`: architecture-description record.

## Generation

From `trinity-rs/`:

```bash
doxygen Doxyfile.trinity-rs
cargo run -p trinity-codegen -- generate
```

The generator scans the original repository tests and Doxygen XML. `#[trinity_test]` is the single semantic test body; Genco emits the `#[kani::proof]` integration targets. It never emits a passing placeholder for an unmigrated legacy test.

## Verification

```bash
cargo fmt --all -- --check
cargo check --workspace
cargo test --workspace
cargo kani --tests -p trinityal
cargo kani --tests -p carbon-trinity-rs
```

Kani's `--tests` mode is required because the generated proof targets are integration-test files guarded with `cfg(kani)`.

## Compatibility rule

A legacy contract is not marked implemented merely because a Rust type with the same name exists. Compatibility evidence requires an executable semantic mapping, or an explicitly classified render-device/screenshot contract when Kani is not the appropriate oracle. See `STATUS.md` and the generated compatibility matrix.
