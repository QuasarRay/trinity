# Trinity-rs Architecture Description Record

- Record identifier: `TRINITY-RS-AD-0001`
- Status: experimental architecture description for `trinity-rs-dev`
- Entity of interest: Carbon-compatible Trinity and Trinity AL reimplementation in Rust
- Architecture baseline: original Trinity commit `a9343c8904310374f284c38a71998af0672b3184`
- Documentation baseline: uploaded `trinity-docs.zip`, SHA-256 `9f77e297e1c0bcb83f0cac2cdb390ac43de9e341db9d11e6e0efce9ad59ec255`
- Architecture description convention: structured to address the concepts publicly described for ISO/IEC/IEEE 42010:2022; this record does not claim third-party certification or reproduce the standard text.

## 1. Entity and purpose

Trinity-rs is an additive Rust reimplementation of Carbon Trinity and Trinity AL. Its purpose is to preserve Carbon-observable names and behavior while replacing the implementation substrate with an opinionated Bevy/Blue-rs architecture and making compatibility evidence machine-generatable.

The original C++/Python implementation remains the behavioral oracle during migration. The architecture explicitly separates *existence of an API-shaped Rust item* from *evidence that the Rust behavior is compatible*.

## 2. Stakeholders and concerns

| Stakeholder | Primary concerns |
|---|---|
| Carbon/Trinity maintainers | Compatibility, recognizable naming, migration cost, upstream change tracking |
| Rust maintainers | Idiomatic ownership, safe Rust, low boilerplate, compile-time generation |
| Graphics programmers | Direct render integration, resource lifetime, render ordering, shader/texture/buffer semantics |
| Python/Carbon consumers | Module/type/method compatibility and gradual coexistence with the original extension |
| Verification reviewers | Traceability from original tests/docs to Rust semantic bodies and generated proofs |
| Build/release maintainers | Reproducible generation, branch isolation, visible gaps rather than false completeness |

## 3. Environment and external entities

Trinity-rs interacts with:

- the original Trinity and Trinity AL source/tests, used as the compatibility oracle;
- Doxygen, which converts source documentation/contracts into XML;
- the machine-readable Sphinx/RST documentation, especially modular assembly and generic post-processing behavior;
- Blue-rs/Blueexposure, providing Carbon lifecycle, reflection metadata and PyO3 conventions;
- Bevy 0.20.0, providing the runtime kernel;
- PyO3 0.30.1, providing Python bindings;
- Genco 0.20.0, providing deterministic source generation;
- Kani, providing bounded model checking for suitable Rust semantic contracts.

## 4. Architecture decisions

### AD-01 — Bevy is the kernel, not an adapter behind Trinity

Production `TrinityPlugins` begins with Bevy's enabled default/render stack, then installs Blueexposure, Blue, Trinity AL and Trinity. Trinity entities are Bevy entities; object state is represented with components/resources; resource objects use Bevy assets; reflection uses Bevy `Reflect`; ordering uses Bevy schedules and system sets.

No second ECS, scheduler, asset graph or render graph is introduced to make the port appear engine-neutral.

### AD-02 — Carbon naming is compatibility surface; Rust implementation remains idiomatic

Public types preserve names such as `EveSOF`, `EveSpaceObject2`, `Tr2BufferAL`, `Tr2TextureAL` and `ALResult`. Rust methods and fields use idiomatic Rust naming internally. PyO3 wrappers can expose Carbon-style names such as `GetResult` without forcing non-idiomatic naming through every Rust call site.

### AD-03 — Blue-rs is integrated, not wrapped away

Every `TrinityClass` registration also creates Blueexposure metadata. Blue lifecycle/exposure plugins are ordered directly in the same Bevy `App`. The current branch vendors the Blue-rs overlay because it is not yet published as a reusable crate in the user's Blue repository.

### AD-04 — Metaprogramming is the default migration mechanism

`trinity-macros` owns derive/attribute macros. Most parsing and token construction is expressed with `proc_macro2`, `syn` and `quote`. Rust's procedural-macro crate rules still require macro definitions to live in a separate proc-macro crate; `proc_macro2` improves reuse/testability but does not remove that language constraint.

`TrinityClass` derives Carbon/Blue registration metadata from type/field annotations. `#[trinity_test]` defines the single semantic test body and its legacy contract identity.

### AD-05 — Doxygen + Genco form the compatibility-generation pipeline

Doxygen extracts machine-readable symbol information from original headers/sources. `trinity-codegen` merges that XML with direct source discovery of GoogleTest/Python tests and Rust `#[trinity_test]` declarations. Genco emits compatibility inventories and Kani targets.

Direct source discovery is intentionally retained because test macros and Python unittest declarations may not survive Doxygen extraction as useful function symbols.

### AD-06 — Kani wrappers are generated only from real semantic bodies

The generator emits `#[kani::proof]` wrappers only for `#[trinity_test(..., verification = "kani")]` bodies. It does not synthesize assertions that automatically pass. Unported original tests remain explicit gaps.

Render-device and screenshot contracts are classified separately because Kani cannot establish hardware/image equivalence merely by wrapping a test name in a proof attribute.

### AD-07 — Verification initialization is narrower than production initialization

`TrinityPlugins` is render-capable and Bevy-centric. `TrinityVerificationPlugins` installs the same Blue/Trinity registration model on `MinimalPlugins + AssetPlugin` so deterministic semantics can be checked without initializing a GPU. This is a verification boundary, not a separate runtime architecture.

## 5. Runtime viewpoint

```text
Bevy App
├── DefaultPlugins / enabled Bevy 3D render stack
├── BlueExposurePlugin
├── BluePlugin
├── TrinityAlPlugin
└── TrinityPlugin
    ├── CarbonHierarchy
    ├── ModularAssembly
    ├── PostProcess
    └── RenderAbstraction
```

Primary runtime model kinds:

- ECS component model: `Eve*`, `Tr2*` state attached to entities.
- Resource model: `EveSOF`, `EveSOFData`, registries and Blue runtime resources.
- Asset model: `Tr2BufferAsset`, `Tr2TextureAsset` and future shader/effect resources.
- Schedule model: ordered `TrinitySet` system sets.
- Reflection model: Bevy `Reflect` plus Carbon/Blue metadata registries.

## 6. Trinity AL viewpoint

Trinity AL is represented as Bevy-native resource abstractions rather than a parallel object system.

Current implemented model:

- `ALResult`: HRESULT-compatible value semantics and category mapping.
- `Tr2BitmapDimensions`: texture dimensions/mip calculations.
- `Tr2VertexDefinition`: vertex declaration state and stream-relative offsets.
- `Tr2BufferAL`: component handle to a Bevy `Asset` containing buffer description/data.
- `Tr2TextureAL`: component handle to a Bevy `Asset` containing texture description/data.
- helper values: viewport, MSAA description, texture coordinate box and subresource range.

Backend-specific DX9/DX11/DX12/Metal behavior remains an explicit migration obligation, not an emulated success path.

## 7. Trinity semantic viewpoint

### Child hierarchy

Carbon owner/parent semantics are represented on Bevy entities. Attaching a child updates the parent relation and propagates the owning `EveShip2` through nested containers. Detaching clears owner state recursively.

### Modular assembly

The RST architecture contract states that part tags persist on `EveChildPartData` and identify all state belonging to a modular part. Trinity-rs therefore stores part-tagged persistent part data directly on `EveSpaceObject2`; editing methods update that data rather than keeping the only authoritative state in a transient editor object.

### Generic post-processing

The documentation states that grouping identity includes shader/effect, quality, execution slot and non-blendable parameters, while blendable parameter values do not split a group. `Tr2PPGenericEffect::grouping_key` implements that contract directly.

## 8. Python interoperability viewpoint

`trinity-py` builds `_trinity_rs`, a development PyO3 module. It intentionally does not overwrite the existing Carbon `trinity` module during the migration. The compatibility shim can be imported explicitly or re-exported into an embedding host.

The end-state compatibility target is Carbon's observable Python surface, but current bindings cover only the first value/runtime types. Missing Python types remain visible in the compatibility inventory.

## 9. Generation and verification viewpoint

```text
Original headers / Python / tests / RST
              │
              ├── Doxygen ──> XML symbol model
              │
              ├── source test discovery
              │
              └── documentation contracts
                         │
                         v
                  trinity-codegen
                  (Genco emitter)
                   │           │
                   │           ├── compatibility matrix / inventory
                   │           └── Doxygen symbol catalogue
                   v
          generated Kani targets
                   │
                   v
            #[kani::proof]
                   │
                   v
        shared #[trinity_test] body
```

Traceability invariant: one Rust semantic body is the source for the ordinary native test and the generated Kani wrapper. A legacy test identity is metadata on that body, not duplicated in a separately maintained proof.

## 10. Correspondence rules

1. Every `TrinityClass` must register the same Carbon-qualified name into `TrinityRegistry` and Blueexposure metadata.
2. Every checked-in native semantic migration test uses `#[trinity_test]`; hand-written Kani proof wrappers are prohibited.
3. Every Kani-generated wrapper must call exactly one generated semantic-body function.
4. A legacy contract may be marked implemented only when its identity maps to an existing semantic body or to a non-Kani evidence class with an explicit oracle.
5. Production Trinity may use the full Bevy render stack; verification must not redefine semantics merely to avoid GPU initialization.
6. Original source/tests are never rewritten as part of the Rust port baseline.

## 11. Architecture risks and unresolved concerns

- Complete Carbon Python API compatibility is not yet implemented.
- Bevy and Trinity have different renderer/resource semantics; some AL APIs will require explicit translation rather than one-to-one wrappers.
- Render-device and screenshot equivalence require runtime/GPU evidence in addition to Kani.
- Doxygen does not encode all behavior; tests and machine-readable narrative docs remain required inputs.
- The vendored Blue-rs dependency should move to a pinned reusable repository/crate once Blue-rs is published.
- Generated inventories must be regenerated when the original baseline changes; a stale inventory is not compatibility evidence.

## 12. Evidence and conformance status

Current source state contains 26 `#[trinity_test]` semantic bodies and 26 checked-in generated Kani wrappers, with no hand-written Kani proof wrappers in Trinity-rs sources.

This architecture description records the intended complete migration architecture, but the implementation is not yet behaviorally equivalent to all of production Trinity/Trinity AL. `STATUS.md` is authoritative for implementation scope. The architecture therefore conforms to its own stated traceability/generation rules for the implemented subset, while full Carbon compatibility remains an open migration objective.
