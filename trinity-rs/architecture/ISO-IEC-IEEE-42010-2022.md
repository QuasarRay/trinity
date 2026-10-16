# Trinity-rs Architecture Description Record

**Standard framing:** ISO/IEC/IEEE 42010:2022 architecture-description concepts  
**System of interest:** Trinity-rs, including Trinity AL compatibility surfaces  
**Repository:** `QuasarRay/trinity`  
**Development branch:** `trinity-rs-dev`  
**Legacy baseline:** `a9343c8904310374f284c38a71998af0672b3184`  
**Record date:** 2026-10-16  
**Authoritative machine-readable documentation SHA-256:** `9f77e297e1c0bcb83f0cac2cdb390ac43de9e341db9d11e6e0efce9ad59ec255`

This document is an architecture description record organized around the concepts of ISO/IEC/IEEE 42010:2022. It is not a third-party certification statement. It records the architecture, rationale, correspondence rules, evidence, and known compatibility gaps of the Rust reimplementation.

## 1. Purpose and system boundary

Trinity-rs reimplements selected Trinity and Trinity AL behavior in safe Rust on top of Bevy while preserving Carbon-facing vocabulary and behavioral contracts. The intended direction is not a generic graphics abstraction that happens to host Carbon compatibility. Bevy is the opinionated runtime kernel and Carbon compatibility is expressed directly through Bevy resources, components, assets, reflection, schedules, and systems.

The system boundary contains:

- the Bevy-native Trinity runtime;
- Trinity AL value/resource compatibility types;
- Carbon/Blue exposure metadata;
- modular space-object semantics;
- child hierarchy semantics;
- generic post-processing semantics;
- the PyO3 compatibility bridge;
- Doxygen/test/documentation discovery;
- Genco-generated inventories and Kani harnesses;
- CI rules that keep generated evidence synchronized.

Legacy C++/Python Trinity, backend implementations, and the original Carbon data/resource ecosystem are external reference systems. They remain compatibility authorities where their observable contracts have not yet been reimplemented.

## 2. Architecture stakeholders and concerns

| Stakeholder | Primary concerns |
|---|---|
| Carbon/Trinity consumers | Carbon names, object behavior, resource conventions, Python-facing semantics, interoperability |
| Trinity-rs maintainers | Low boilerplate, convention over configuration, generated code, traceability to legacy contracts |
| Bevy integrators | Native `App`/`World`/asset/reflection/schedule semantics; no unnecessary parallel engine layer |
| Verification maintainers | Exact test provenance, deterministic Kani boundaries, no placeholder proofs, reproducible generated evidence |
| Graphics maintainers | Correct render-world/device behavior, resource lifetime, shader/effect semantics, backend evidence |
| Blue consumers | One exposure/metadata model shared with the Bevy runtime |
| Python consumers | Stable Carbon method/type names and compatible observable behavior for implemented surfaces |

The dominant architecture tension is between **deep Bevy integration** and **Carbon compatibility**. The chosen approach treats Carbon semantics as constraints on Bevy-native types and systems, rather than preserving a C++-shaped internal architecture merely for structural similarity.

## 3. Architecture principles

1. **Bevy is the kernel.** Do not introduce an engine-independent runtime facade merely to isolate Bevy.
2. **Carbon observable behavior is the compatibility target.** Legacy names, semantics, persistence rules, and Python-visible contracts take precedence over inventing a cleaner but incompatible API.
3. **Generation is preferred to repetition.** Repeated metadata, inventories, Kani wrappers, and compatibility reports belong in derive/attribute macros or Genco.
4. **Convention over configuration.** Carbon types register through a common class convention; semantic tests declare provenance once; the generator derives the rest.
5. **Machine-readable sources are first-class.** Doxygen XML, original tests, and pinned RST documentation participate in compatibility discovery.
6. **Verification classes remain distinct.** Pure deterministic contracts may use Kani. Render-device, screenshot, and integration behavior retain their appropriate evidence class.
7. **No false completeness.** Unimplemented legacy contracts remain explicit in generated reports.

## 4. Runtime viewpoint

### 4.1 Bevy application composition

Production composition is:

`DefaultPlugins -> BlueExposurePlugin -> BluePlugin -> TrinityAlPlugin -> TrinityPlugin`

Verification composition uses `MinimalPlugins` plus Bevy's `AssetPlugin`, but uses the same Trinity/Blue registration paths. There is no separate mock object model.

The runtime establishes ordered Bevy `SystemSet`s:

- `CarbonHierarchy`;
- `ModularAssembly`;
- `PostProcess`;
- `RenderAbstraction`.

These sets define opinionated ordering and provide the extension points for further Carbon semantics.

### 4.2 Carbon types in the ECS

Carbon concepts are represented according to their runtime role:

- persistent or global datasets as Bevy `Resource`s;
- scene/object behavior as Bevy `Component`s;
- GPU/resource objects with asset identity as Bevy `Asset`s where appropriate;
- small AL descriptions/enums as reflected Rust value types.

This is intentionally stronger integration than wrapping a separate Trinity object graph inside Bevy.

## 5. Blue and metadata viewpoint

`trinity-meta` defines the shared compatibility metadata model. `TrinityAppExt::register_trinity_class<T>()` registers Bevy reflection metadata, stores `TrinityClassInfo`, and mirrors compatible class information into the Blue registry.

`#[derive(TrinityClass)]` is the default class declaration mechanism. It uses `proc_macro2`, Syn, and Quote to generate:

- Carbon module/name identity;
- Rust identity;
- legacy-header provenance;
- class kind;
- readable/writable/persisted field metadata.

The resulting rule is:

> A Carbon-exposed class should declare compatibility metadata once at the Rust type and be registered through the common Bevy/Blue path.

Manual duplicate registries are an architecture violation unless a specific legacy behavior cannot be represented by the common model.

## 6. Trinity AL viewpoint

Trinity AL is implemented as Bevy-compatible Rust types rather than a verbatim backend hierarchy.

Current deterministic compatibility includes:

- HRESULT/result categorization and success semantics;
- `Tr2BitmapDimensions` mip/array/compression behavior;
- `Tr2VertexDefinition` data-type sizing, offsets, search, comparison, and movement semantics;
- Carbon pixel-format names required by tested behavior;
- `Tr2TextureSubresource` legacy sentinel ranges, clamping, validity, full-resource coverage, cropping, and mip advancement;
- representative buffer/texture asset/resource surfaces.

All **49 / 49** original tests in the four deterministic Trinity AL suites are mapped. Regeneration uses `--deny-unmapped-kani`; dropping one of these mappings fails generation.

This does not imply complete Trinity AL device equivalence. Backend-coupled render contexts, fences, queries, shaders, swap chains, ray tracing, upload/readback paths, and device-specific lifetime rules remain separate render-device obligations.

## 7. Trinity semantic viewpoint

### 7.1 Modular assembly

The pinned `modular-assembly.rst` establishes cross-cutting contracts that are not captured by a single C++ file. Trinity-rs models them directly in Bevy:

- `EveSOF` and `EveSOFData` are resources carrying SOF-side data;
- `EveSpaceObject2` is a Bevy component containing modular object state;
- `EveChildPartData` persists per-part state;
- `EveModularObjectModifier` is a transient editing interface over the Bevy `World`;
- part tags identify a part across persisted state, locators, mesh instances, and logical children;
- add/remove/transform operations update the object model and recompute the currently implemented bounds/locator views;
- empty faction/race inputs inherit the object defaults;
- unknown or already-removed tags retain Carbon-style error behavior.

The generated evidence maps **38 / 49** original modular Python tests plus two documentation-derived modular contracts. The remaining 11 modular Python contracts are explicit obligations.

One documented semantic deserves special attention: legacy documentation states that modifier edits are immediate but culling bounds commit through `ApplyBounds` or modifier lifetime completion. Where the current Rust implementation recomputes bounds more eagerly for deterministic modeling, that difference must remain visible until the lifetime/commit contract is matched exactly.

### 7.2 Child hierarchy

Carbon child/container behavior is implemented through Bevy entities/components and explicit part-tag propagation. **6 / 10** focused child-hierarchy Python tests are currently mapped. Remaining hierarchy/resource-loading behavior stays in the compatibility matrix.

### 7.3 Generic post-processing

The pinned `generic-pp-effects.rst` defines custom effect identity, grouping, blendable parameters, quality, ordering, and execution slots. Current Trinity-rs models:

- Carbon `Tr2PPExecutionSlot` names;
- `Tr2PPParameter`;
- `Tr2PPGenericEffect`;
- group identity based on effect, quality, execution slot, and non-blendable parameters;
- blendable parameters as values that do not split an otherwise identical group.

Two documentation-derived contracts exercise identical-effect grouping and non-blendable parameter separation. Actual shader/effect execution remains a render-device concern.

## 8. Python interoperability viewpoint

`trinity-py` is a PyO3 bridge over the Rust runtime. Implemented surfaces preserve Carbon-style type/method names where exposed, for example `ALResult` and methods such as `GetResult`.

The bridge deliberately delegates behavior to Rust types; it must not become a second implementation of Trinity semantics.

Current limitations:

- the module is a development `_trinity_rs` bridge, not yet the complete production `trinity` namespace;
- most legacy Python classes/functions are not yet exposed;
- binary extension ABI compatibility with historical builds is not claimed.

## 9. Generation and verification viewpoint

### 9.1 Compatibility discovery

`trinity-codegen` discovers:

1. Doxygen symbols from generated XML;
2. C++ GoogleTest declarations;
3. Python `test_*` declarations;
4. Rust `#[trinity_test]` semantic bodies.

The uploaded documentation archive is pinned by SHA-256 in the generator. The supplied archive matches that hash exactly.

### 9.2 Semantic-test convention

A semantic test is declared once:

`#[trinity_test(legacy = "...", verification = "...")]`

The proc macro emits:

- the ordinary Rust test;
- a hidden reusable semantic body;
- a machine-readable test descriptor.

Genco then emits Kani harnesses that call those bodies. Kani wrappers are therefore generated from real semantic tests, not handwritten copies.

### 9.3 Evidence inventory

Current generated inventory:

| Evidence | Count |
|---|---:|
| Doxygen symbols | 15,379 |
| Discovered original tests | 328 |
| Rust semantic mappings | 97 |
| Mapped discovered original tests | 93 |
| Documentation-derived contracts | 4 |
| Genco-generated Kani wrappers | 97 |
| Duplicate mappings | 0 |

Original-test classification and coverage:

| Verification class | Original | Mapped | Remaining |
|---|---:|---:|---:|
| Deterministic / Kani | 49 | 49 | 0 |
| Integration | 83 | 44 | 39 |
| Render-device | 196 | 0 | 196 |
| **Total** | **328** | **93** | **235** |

All 97 current Rust semantic tests explicitly opt into generated Kani wrappers. This is **harness-generation coverage**, not evidence that all 97 proofs have completed successfully. The generator's automatic deterministic completeness gate applies to the 49 original Kani-suitable AL contracts.

### 9.4 CI evidence lifecycle

The `Trinity-rs` workflow:

1. regenerates Doxygen XML and Genco outputs;
2. rejects missing deterministic mappings;
3. uploads generated evidence;
4. formats;
5. checks all targets;
6. runs strict Clippy with warnings denied;
7. runs the Rust workspace tests;
8. synchronizes generated evidence back to `trinity-rs-dev` when it changed.

Kani proof execution is a separate explicit workflow-dispatch job. This separation prevents an ordinary native CI pass from being reported as formal proof completion.

## 10. Correspondence rules

The following rules connect architecture views and are intended to be mechanically enforceable over time:

- **CR-01 — Carbon class identity:** every exposed `TrinityClass` has one Carbon module/name and one legacy provenance path.
- **CR-02 — Blue correspondence:** a class registered through `TrinityAppExt` appears in both Trinity and Blue registries.
- **CR-03 — Bevy ownership:** runtime state belongs to Bevy `World`/resources/components/assets rather than a duplicate hidden world.
- **CR-04 — Test provenance:** every generated Kani harness originates from a real `#[trinity_test]` semantic body.
- **CR-05 — Deterministic completeness:** every original test auto-classified as Kani-suitable must have a mapped Rust semantic body.
- **CR-06 — Render honesty:** a render-device/screenshot obligation must not be reclassified as deterministic solely to increase formal-coverage counts.
- **CR-07 — Generated ownership:** inventories, compatibility matrices, pairing reports, and generated Kani targets are produced by `trinity-codegen`/Genco, not manually maintained.
- **CR-08 — Python delegation:** Python compatibility methods delegate to Rust semantics rather than reproducing behavior independently.
- **CR-09 — Carbon vocabulary:** compatibility-facing public names preserve Carbon naming unless a separate idiomatic Rust convenience layer can coexist without ambiguity.
- **CR-10 — Branch isolation:** experimental reimplementation changes live on `trinity-rs-dev`; `main` is not modified by this effort.

## 11. Architecture decisions

### AD-01 — Bevy as the opinionated kernel

**Decision:** integrate directly with Bevy rather than introduce a neutral engine interface.  
**Rationale:** reduces duplicated infrastructure and makes assets, ECS, reflection, scheduling, rendering, and plugin composition reusable immediately.  
**Trade-off:** Bevy API evolution becomes a direct maintenance concern.

### AD-02 — Carbon compatibility is behavioral, not structural

**Decision:** preserve observable Carbon contracts and names without requiring the Rust internals to mimic C++ class hierarchy/layout.  
**Rationale:** enables idiomatic Rust and Bevy while retaining ecosystem interoperability.  
**Trade-off:** binary C++ ABI compatibility is not implied.

### AD-03 — Blue shares the same runtime

**Decision:** Blue exposure is registered through the same Bevy application and metadata path.  
**Rationale:** avoids drift between engine state and exposure state.

### AD-04 — Metaprogramming before manual boilerplate

**Decision:** use derive/attribute/declarative macros, delegation, and Genco for repetitive structure.  
**Rationale:** compatibility work scales only if adding a class/test updates metadata and verification evidence by convention.

### AD-05 — Original machine-readable material is input data

**Decision:** Doxygen XML, tests, and the pinned documentation archive drive inventories and compatibility traceability.  
**Rationale:** reduces dependence on manually curated compatibility spreadsheets and prose.

### AD-06 — Shared semantic bodies generate Kani wrappers

**Decision:** native tests and Kani wrappers execute the same Rust semantic body.  
**Rationale:** prevents hand-maintained native/proof test divergence.

### AD-07 — Verification follows behavior class

**Decision:** deterministic value logic, integration semantics, render-device behavior, and screenshots remain distinct evidence classes.  
**Rationale:** Kani is valuable for pure bounded semantics but cannot substitute for actual GPU/render-world evidence.

### AD-08 — Generated evidence is CI-owned

**Decision:** CI regenerates, publishes, validates, and synchronizes generated compatibility artifacts.  
**Rationale:** stale checked-in reports must not be treated as authoritative.

### AD-09 — Python is an interoperability boundary

**Decision:** expose implemented Rust behavior through PyO3 rather than reimplementing Carbon semantics in Python glue.  
**Rationale:** one semantic implementation reduces divergence.

## 12. Known risks and residual obligations

### R-01 — Render-device gap

196 original tests are classified as render-device and remain unmapped. Full production compatibility requires actual Bevy render-world/wgpu implementation and appropriate GPU/image evidence.

### R-02 — Integration gap

39 original integration tests remain unmapped, including remaining modular/child behavior and other Python-facing contracts.

### R-03 — Harness generation versus proof execution

97 Kani wrappers are generated, but generated wrapper count is not equivalent to proof-success count. Formal status must cite actual Kani execution artifacts.

### R-04 — Doxygen ambiguity

The legacy source emits Doxygen warnings for ambiguous or undocumented declarations. Inventory generation tolerates those warnings, so symbol completeness should be interpreted as discovered Doxygen output, not a proof that legacy documentation itself is internally consistent.

### R-05 — Bevy version coupling

Deep integration intentionally accepts Bevy coupling. Compatibility code should prefer project-level macros/conventions over generic abstraction layers when Bevy APIs change.

### R-06 — Python surface incompleteness

The PyO3 bridge does not yet expose the entire Carbon `trinity` namespace and should not be advertised as a drop-in module replacement.

## 13. Evolution rules

A change is architecture-conformant when it:

- keeps Bevy as the runtime owner;
- preserves or deliberately versions Carbon-observable behavior;
- adds reusable macro/codegen infrastructure when repetition becomes systematic;
- maps new deterministic legacy contracts before weakening a verification gate;
- adds render-device evidence rather than pretending device behavior is pure;
- regenerates evidence from source;
- updates this record when architecture decisions, boundaries, or evidence classes materially change.

The generated compatibility matrix is the operational backlog. “Full compatibility” becomes supportable only when the relevant original contracts are implemented with the correct evidence class and the remaining ecosystem/ABI requirements are explicitly closed.
