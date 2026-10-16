# Trinity-rs implementation status

**Snapshot:** 2026-10-16  
**Development branch:** `trinity-rs-dev`  
**Legacy baseline:** `a9343c8904310374f284c38a71998af0672b3184`  
**Machine-readable documentation SHA-256:** `9f77e297e1c0bcb83f0cac2cdb390ac43de9e341db9d11e6e0efce9ad59ec255`

This status is evidence-oriented. Trinity-rs is a substantial Carbon-compatible Rust/Bevy reimplementation, but it is **not yet a complete production-equivalent replacement for legacy Trinity/Trinity AL**. The generated compatibility inventory keeps the remaining obligations explicit rather than converting them into placeholder implementations or placeholder proofs.

## Implemented architecture

- Bevy 0.20 is the runtime kernel. Trinity-rs uses Bevy `App`, `World`, resources, components, assets, reflection, plugin groups, and ordered `SystemSet`s directly.
- Blue-rs is integrated into the same Bevy application. Carbon class registration populates both the Trinity metadata registry and Blue registry; there is no parallel compatibility runtime.
- `#[derive(TrinityClass)]` generates Carbon/Blue metadata with `proc_macro2`, Syn, and Quote.
- `#[trinity_test]` generates a native Rust test plus a reusable semantic body. Genco consumes the discovered semantic bodies and emits Kani harnesses.
- `delegate` is used for narrow forwarding surfaces where delegation reduces boilerplate without hiding Carbon semantics.
- Doxygen XML, original GoogleTests/Python tests, and the pinned machine-readable RST documentation are treated as compatibility inputs.
- PyO3 exposes a Rust-backed compatibility boundary for implemented Carbon types; it remains a development bridge, not yet a drop-in replacement for the complete production `trinity` Python module.

## Generated evidence

The current Doxygen/Genco inventory contains:

| Evidence | Count |
|---|---:|
| Doxygen symbols | 15,379 |
| Original discovered tests | 328 |
| Rust semantic mappings | 97 |
| Mappings to discovered original tests | 93 |
| Documentation-derived semantic contracts | 4 |
| Genco-generated Kani wrappers | 97 |
| Duplicate semantic mappings | 0 |

Original tests are classified by the generator as:

| Suggested verification | Original tests | Mapped |
|---|---:|---:|
| Deterministic / Kani | 49 | 49 |
| Integration | 83 | 44 |
| Render-device | 196 | 0 |
| **Total** | **328** | **93** |

The four additional mapped contracts come from the pinned RST documentation:

- modular part tags remain stable across edit sessions;
- modular per-part transforms persist;
- identical generic post-process effects group;
- non-blendable generic post-process parameters split groups.

## Trinity AL deterministic contract coverage

All original contracts in the four deterministic Trinity AL suites are mapped:

| Legacy suite | Mapped |
|---|---:|
| `ALResultTest.cpp` | 8 / 8 |
| `BitmapDimensions.cpp` | 7 / 7 |
| `VertexDefinition.cpp` | 7 / 7 |
| `TextureSubresource.cpp` | 27 / 27 |
| **Total** | **49 / 49** |

`scripts/regenerate.sh` runs the generator with `--deny-unmapped-kani`, so losing any auto-classified deterministic mapping is a CI failure.

Notable compatibility work includes Carbon HRESULT/category behavior, bitmap/mip rules, vertex-definition sizing/search/equality, typeless/compressed pixel formats, and legacy `Tr2TextureSubresource` sentinel/clamp/full/valid/crop/advance behavior.

## Trinity semantic coverage

Implemented Bevy-native semantic areas include:

- Carbon child/container hierarchy and part-tag propagation;
- modular `EveSOF` / `EveSpaceObject2` assembly;
- persistent `EveChildPartData`;
- static-mesh instance grouping and animated-hull accounting;
- modular add/remove/reopen/transform behavior;
- bounds and shape-ellipsoid recomputation;
- damage-locator merging and transform propagation;
- logical audio-placement containers;
- default faction/race behavior;
- generic post-process execution slots and grouping identity;
- grouping rules that distinguish blendable from non-blendable parameters.

Current focused Python-contract mapping:

- modular assembly: **38 / 49**;
- child hierarchy: **6 / 10**.

## Generation and CI policy

`trinity-codegen` owns generated compatibility evidence. CI:

1. generates Doxygen XML;
2. regenerates inventories and Kani targets with Genco;
3. uploads generated contract evidence as an Actions artifact;
4. runs `cargo fmt --all`;
5. runs `cargo check --workspace --all-targets`;
6. runs strict `cargo clippy --workspace --all-targets -- -D warnings`;
7. runs `cargo test --workspace`;
8. synchronizes checked-in generated evidence on `trinity-rs-dev` when the generated outputs changed.

The Kani workflow is configured as an explicit `workflow_dispatch` job. The repository therefore distinguishes **generated harness coverage** from **a completed Kani proof run**. The presence of 97 generated wrappers must not be interpreted as evidence that all 97 proofs have completed successfully.

## Remaining compatibility obligations

The generated inventory currently leaves **235 original tests unmapped**:

- 39 integration contracts;
- 196 render-device contracts.

The largest remaining scope is production rendering/GPU equivalence: shader/effect execution, resource lifetime and upload/readback behavior, render contexts, swap chains, fences, queries, ray tracing, backend-specific semantics, image/screenshot behavior, and other device-coupled contracts. These require Bevy/wgpu/render-world integration and appropriate render-device or screenshot evidence; they should not be converted into fake Kani proofs.

Other remaining boundaries include the full production Python namespace/API surface, resource serialization/loading equivalence, and any binary C++ ABI compatibility required by existing Carbon consumers.

## Current validation

For the pre-record code head `8b724d67ce26f8eb7a2aa8defa0fb220208066d0`, Doxygen/Genco generation, `cargo check --workspace --all-targets`, and strict Clippy completed successfully in GitHub Actions before this record was written. The full workspace test stage was still executing when the record commit was prepared; the record commit itself triggers the same CI pipeline again.

See `generated/inventory.json`, `generated/compatibility-matrix.md`, and `generated/kani-pairing.md` for machine-generated detail.
