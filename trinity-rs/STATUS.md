# Trinity-rs implementation status

- Branch scope: `trinity-rs-dev` only
- Original compatibility baseline: `a9343c8904310374f284c38a71998af0672b3184`
- Uploaded documentation SHA-256: `9f77e297e1c0bcb83f0cac2cdb390ac43de9e341db9d11e6e0efce9ad59ec255`
- Runtime kernel: Bevy 0.20.0
- Python bridge: PyO3 0.30.1
- Generator: Doxygen XML + source discovery + Genco 0.20.0
- Verification model: shared semantic bodies + generated Kani wrappers
- Current status: experimental compatibility slice; **not full production equivalence**

## Implemented architecture

- Additive `trinity-rs/` Cargo workspace; original C++/Python sources are unchanged and remain the oracle.
- Bevy-native runtime integration: components, resources, assets, reflection, schedules/system sets, and production render plugin stack.
- Vendored Blue-rs compatibility kernel sharing the same Bevy `App`, `World`, type registration and lifecycle.
- Carbon-named Trinity and Trinity AL Rust types.
- `proc_macro2`/Syn/Quote derive and attribute macros for class metadata and shared semantic tests.
- `delegate` for direct delegation to Bevy/runtime/registry containers where it removes boilerplate.
- Doxygen and direct source discovery for C++ GoogleTests, Python tests and symbols.
- Genco emission of inventories, compatibility reports and Kani integration targets.
- Branch-local GitHub Actions for generation, formatting, check, Clippy and tests; optional Kani workflow dispatch.
- ISO/IEC/IEEE 42010:2022-style architecture description record.

## Executable semantic mappings

The initial slice contains **26** shared Rust semantic tests and **26** generated Kani wrappers:

- Trinity child hierarchy: 6
- Trinity modular assembly: 2
- Trinity generic post-processing: 2
- Trinity AL bitmap/mip behavior: 7
- Trinity AL result semantics: 7
- Trinity AL vertex definition semantics: 2

These numbers describe implemented semantic mappings, not the size of the original test suite. The generator inventories the wider suite and leaves unmapped contracts explicit.

## Implemented Trinity AL substrate

Current Rust types include:

- `ALResult`
- `Tr2BitmapDimensions`
- `Tr2VertexDefinition`
- `Tr2BufferAL`
- `Tr2TextureAL`
- viewport/MSAA/subresource helper values
- Bevy asset-backed buffer/texture state

## Open compatibility obligations

The following are intentionally not represented as complete merely because a Rust scaffold exists:

- complete SOF/modular assembly behavior;
- complete Carbon Python module/API surface;
- complete Trinity AL render-context and device APIs;
- shader/effect compiler compatibility;
- complete pixel-format and resource semantics;
- DX/Metal/backend-specific behavior;
- swap chains, fences, queries and ray tracing;
- resource serialization/loading parity;
- GPU runtime tests;
- screenshot/image equivalence;
- performance and production rollout evidence.

Kani is not used as a substitute for GPU/device/image equivalence. Those original tests remain classified as runtime/render evidence obligations.

## Validation state

The local execution environment used to author this branch does not provide `cargo`, `rustc` or `doxygen`. Consequently, source/tree consistency was checked before publication, while compile/test validation is delegated to the branch-local GitHub Actions workflow. Any failing CI result must be treated as an implementation defect rather than waived as equivalent behavior.
