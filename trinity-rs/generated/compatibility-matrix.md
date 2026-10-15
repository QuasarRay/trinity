# Trinity-rs compatibility evidence

This checked-in record describes the bootstrap subset. The complete matrix is regenerated from the pinned checkout by `trinity-codegen`.

| Area | Evidence | Status |
|---|---|---|
| Child hierarchy | original Python tests → shared Rust bodies → generated Kani | mapped subset |
| Modular assembly | machine-readable RST → shared Rust bodies → generated Kani | mapped subset |
| Generic post-processing | machine-readable RST grouping rules → generated Kani | mapped subset |
| ALResult | original GoogleTests → generated Kani | mapped subset |
| Bitmap/mip semantics | original GoogleTests → generated Kani | mapped subset |
| Vertex definitions | original GoogleTests → generated Kani | mapped subset |
| GPU/device/render behavior | original device tests and screenshots | explicit non-Kani obligation |
| Complete Carbon Python API | original module behavior | open migration obligation |
