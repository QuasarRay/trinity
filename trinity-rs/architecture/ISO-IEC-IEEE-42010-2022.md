# Trinity-rs Architecture Description Record

- Standard profile: ISO/IEC/IEEE 42010:2022 architecture-description concepts
- System of interest: Trinity-rs and Trinity AL-rs
- Repository: `QuasarRay/trinity`
- Development branch: `trinity-rs-dev`
- Legacy compatibility baseline: `a9343c8904310374f284c38a71998af0672b3184`
- Machine-readable documentation baseline SHA-256: `9f77e297e1c0bcb83f0cac2cdb390ac43de9e341db9d11e6e0efce9ad59ec255`
- Status: experimental migration architecture; not a claim of complete production equivalence

## 1. Purpose and architecture objective

Trinity-rs reimplements the observable Carbon Trinity and Trinity AL contracts in Rust while using Bevy as the runtime kernel. The architecture preserves Carbon public names and compatibility obligations, but it does not reproduce the C++ implementation structure when Bevy already supplies the corresponding runtime mechanism.

The original headers, Doxygen XML, machine-readable RST documentation, Python tests, GoogleTest tests, and the pinned legacy commit form the compatibility oracle. Generated metadata and verification artifacts are evidence derived from those sources; they are not independent specifications.

## 2. Stakeholders and concerns

| Stakeholder | Primary concern |
|---|---|
| Carbon/Trinity maintainers | Behavioral compatibility and migration traceability |
| Rust maintainers | Idiomatic Rust, minimal manual boilerplate, safe code |
| Bevy maintainers/integrators | Native use of ECS, assets, reflection, schedules and rendering |
| Python/Carbon consumers | Preservation of Carbon names and observable Python behavior |
| Verification reviewers | One-to-one traceability from legacy tests to executable Rust contracts |
| Graphics/runtime engineers | Clear separation of deterministic semantics from GPU/device evidence |

## 3. Environment and external dependencies

The system is embedded in the Carbon ecosystem and interoperates with:

- original Trinity and Trinity AL source/tests as the migration oracle;
- Blue-rs/Blueexposure compatibility metadata and lifecycle registration;
- Bevy 0.20.0 as ECS, app, schedule, reflection, asset and render substrate;
- PyO3 0.30.1 for the development Python compatibility module;
- Doxygen XML for machine-readable original symbol extraction;
- Genco 0.20.0 for generated compatibility and verification artifacts;
- Kani for bounded verification of deterministic semantic contracts.

## 4. Architecture decisions

### AD-01 — Bevy is the kernel

Trinity-rs does not place a neutral engine abstraction between Trinity and Bevy. Production `TrinityPlugins` installs Bevy's enabled default/render stack and then installs Blue, Trinity AL and Trinity directly. Trinity objects are Bevy components/resources/assets; ordering uses Bevy schedules/system sets; reflection uses Bevy `Reflect`.

### AD-02 — Carbon names are the compatibility surface

Public compatibility types retain names such as `EveSOF`, `EveSpaceObject2`, `Tr2BufferAL`, `Tr2TextureAL`, `Tr2VertexDefinition` and `ALResult`. Rust implementation fields and methods remain idiomatic. PyO3 wrappers may expose Carbon-style method spellings without forcing those spellings through the Rust implementation.

### AD-03 — Blue-rs is a first-class Bevy participant

Trinity class registration feeds both `TrinityRegistry` and `BlueRegistry`. Blue exposure/lifecycle plugins share the same Bevy `App`; no second object registry or scheduler is introduced. The branch vendors the Blue-rs compatibility kernel until the standalone Blue Rust port is published as a reusable crate.

### AD-04 — Metaprogramming is the migration default

`trinity-macros` uses `proc_macro2`, `syn` and `quote` to derive Carbon class metadata and to create the shared semantic body used by ordinary tests and generated Kani harnesses. Rust's proc-macro crate boundary is retained because `proc_macro2` does not permit a procedural macro to be defined and consumed in the same ordinary crate.

### AD-05 — Doxygen plus source discovery form the compatibility model

Doxygen extracts symbols from original Trinity/Trinity AL headers and sources. Direct discovery supplements Doxygen for GoogleTest macro invocations and Python `test_*` methods. The generator merges these with Rust `#[trinity_test]` declarations.

### AD-06 — Genco owns generated verification artifacts

`trinity-codegen` generates compatibility inventories and Kani integration targets. A Kani proof wrapper is generated only for an existing semantic body classified as `verification = "kani"`. The generator does not create placeholder proofs for unmigrated tests.

### AD-07 — GPU/image contracts are not misrepresented as Kani proofs

Render-device, shader, swap-chain, ray-tracing and screenshot contracts remain explicit non-Kani obligations. Kani establishes deterministic Rust semantics; device/image equivalence requires runtime GPU evidence.

## 5. Runtime viewpoint

```text
Bevy App
├── DefaultPlugins / Bevy render stack
├── BlueExposurePlugin
├── BluePlugin
├── TrinityAlPlugin
└── TrinityPlugin
    ├── CarbonHierarchy
    ├── ModularAssembly
    ├── PostProcess
    └── RenderAbstraction
```

Production and verification use the same Carbon registrations. `TrinityVerificationPlugins` deliberately initializes a narrower Bevy host (`MinimalPlugins + AssetPlugin`) so deterministic tests do not require a GPU. This is a verification initialization boundary, not a second runtime architecture.

## 6. Trinity AL viewpoint

Current Bevy-native AL mappings include:

- `ALResult`: HRESULT-compatible result/category behavior;
- `Tr2BitmapDimensions`: dimensions, true mip count and mip sizes;
- `Tr2VertexDefinition`: declaration items and stream-relative offsets;
- `Tr2BufferAL`: Bevy asset-backed buffer state;
- `Tr2TextureAL`: Bevy asset-backed texture state;
- helper values such as viewport, MSAA description, texture coordinate boxes and subresources.

Backend-specific DX/Metal behavior is an explicit migration obligation and must not be represented by unconditional success stubs.

## 7. Trinity semantic viewpoint

### Child hierarchy

Owner/parent state is attached to Bevy entities. Adding an effect child assigns the owning ship; adding to a nested container assigns the parent and propagates the owner's entity. Detaching recursively clears ownership.

### Modular assembly

Part tags are persistent compatibility identifiers. `EveSpaceObject2` stores part-tagged `EveChildPartData`; transform edits update persistent part state rather than transient editor-only state.

### Generic post-processing

Grouping identity includes effect, quality, execution slot and non-blendable parameter values. Blendable values do not split a group. This rule is derived from the uploaded machine-readable Trinity documentation.

## 8. Python interoperability viewpoint

`trinity-py` builds the development module `_trinity_rs`. It exposes Carbon-compatible names for the currently implemented types while deliberately avoiding replacement of the production `trinity` module before equivalence evidence exists. An embedding host may re-export verified types into the Carbon namespace.

## 9. Generation and verification viewpoint

```text
Original headers / C++ tests / Python tests / RST
          │
          ├── Doxygen ──> XML symbols
          ├── direct test discovery
          └── machine-readable contracts
                       │
                       v
                trinity-codegen
             (Genco generation)
                 │          │
                 │          └── compatibility inventory
                 v
         generated Kani target
                 │
                 v
           #[kani::proof]
                 │
                 v
       shared #[trinity_test] body
```

A native semantic test and its Kani harness execute the same body. Hand-written duplicate Kani bodies are prohibited by architecture policy.

## 10. Correspondence rules

1. Every derived `TrinityClass` has one Carbon-qualified identity.
2. Registration of a Trinity class must update Trinity and Blue registries consistently.
3. Every implemented deterministic legacy contract is identified by a `#[trinity_test(legacy = ...)]` mapping.
4. Every Kani wrapper invokes exactly one shared semantic body.
5. Unmapped original tests remain visible as migration obligations.
6. Render-device/screenshot obligations require their own runtime oracle.
7. Original C++/Python sources remain unchanged by the Rust-port branch.

## 11. Current evidence and open obligations

The initial implementation contains 26 mapped semantic tests: 10 Trinity contracts and 16 Trinity AL contracts, with generated Kani wrappers for those Kani-suitable bodies. It also provides the generation pipeline that inventories the wider legacy suite.

This is not full production equivalence. Remaining obligations include complete SOF/modular behavior, complete Python exposure, the full AL/render-context API, complete formats, shader/effect compilation, backend-specific rendering, swap chains, ray tracing, resource serialization, device tests and screenshot equivalence.

The architecture is designed so those gaps are machine-enumerable rather than hidden by placeholder APIs. `STATUS.md` is authoritative for implemented scope.
