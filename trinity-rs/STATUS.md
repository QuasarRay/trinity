# Trinity-rs status

## Baseline

- Branch: `trinity-rs-dev`
- Legacy commit: `a9343c8904310374f284c38a71998af0672b3184`
- Main branch modified: **no**
- Machine-readable documentation archive SHA-256: `9f77e297e1c0bcb83f0cac2cdb390ac43de9e341db9d11e6e0efce9ad59ec255`

## Implemented architecture

- Bevy 0.20.0 is the runtime/ECS/asset/reflection/render kernel.
- Blue-rs is vendored from the previously implemented Blue/Blueexposure Rust overlay.
- Trinity/Trinity AL registration flows into both `TrinityRegistry` and Blueexposure's `BlueRegistry`.
- Public compatibility names preserve Carbon naming (`Eve*`, `Tr2*`, `ALResult`).
- PyO3 development bindings expose the first compatibility types without replacing the existing Carbon `trinity` module in-place.
- Doxygen XML and original source tests are machine-discovered by `trinity-codegen`.
- Genco is the canonical emitter for inventories and Kani wrappers.
- `#[trinity_test]` produces one native semantic body; generated `#[kani::proof]` wrappers call that same body.

## Implemented semantic surface

Trinity AL:

- `ALResult` status/category/error-message semantics.
- `Tr2BitmapDimensions` width/height/depth/mip-count/mip-size behavior for the currently represented formats.
- `Tr2VertexDefinition` item layout and stream-relative offsets.
- `Tr2BufferAL` as a Bevy asset-backed buffer abstraction.
- `Tr2TextureAL` as a Bevy asset-backed texture abstraction.
- `Tr2Viewport`, `Tr2MsaaDesc`, texture coordinate boxes and subresource ranges.

Trinity:

- `EveSpaceObjectChild`, `EveChildMesh`, `EveChildContainer`, `EveShip2` hierarchy ownership/parent propagation on Bevy entities.
- `EveSOFData`, `EveSOF`, `EveSpaceObject2` and persistent modular part-tag/transform state.
- `Tr2PPGenericEffect` grouping rules derived from the machine-readable generic post-processing documentation.
- Production and verification plugin groups with different initialization breadth but the same Carbon registrations.

## Verification inventory in this commit

- Native semantic `#[trinity_test]` bodies: **26**
- Checked-in generated Kani wrappers: **26**
- Hand-written `#[kani::proof]` wrappers: **0**

The full count of original legacy tests is deliberately generated from the repository checkout rather than hard-coded here. `cargo run -p trinity-codegen -- generate` inventories all discoverable GoogleTest and Python tests and classifies render-device/screenshot contracts separately.

## Not yet equivalent to production Trinity

This commit does **not** claim complete behavioral equivalence with the entire original Trinity/Trinity AL codebase. Major remaining obligations include the complete render-context API, all pixel/texture formats, shader/effect compilation, backend-specific DX/Metal behavior, ray tracing, swap chains, complete SOF/modular assembly behavior, resource serialization, the full Python exposure surface, all render-device tests and screenshot equivalence.

The architecture is intended to make those obligations machine-enumerable and progressively provable instead of hiding them behind placeholder APIs.
