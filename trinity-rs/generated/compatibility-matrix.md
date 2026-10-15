# Bootstrap compatibility matrix

This file records the semantic mappings already implemented in this commit. Running `trinity-codegen` against the complete repository replaces it with the full discovered legacy-test matrix.

| Legacy contract | Port status | Verification |
|---|---|---|
| `trinity/tests/Python/tests/test_childhierarchy.py::TestChildHierarchy::test_initValues` | implemented semantic mapping | `kani` |
| `trinity/tests/Python/tests/test_childhierarchy.py::TestChildHierarchy::test_ownerAssignment` | implemented semantic mapping | `kani` |
| `trinity/tests/Python/tests/test_childhierarchy.py::TestChildHierarchy::test_transitiveOwnerAssignment` | implemented semantic mapping | `kani` |
| `trinity/tests/Python/tests/test_childhierarchy.py::TestChildHierarchy::test_parentAssignment` | implemented semantic mapping | `kani` |
| `trinity/tests/Python/tests/test_childhierarchy.py::TestChildHierarchy::test_ownerRemoval` | implemented semantic mapping | `kani` |
| `trinity/tests/Python/tests/test_childhierarchy.py::TestChildHierarchy::test_parentRemoval` | implemented semantic mapping | `kani` |
| `doc/source/eve/modular-assembly.rst::part-tags-stay-valid-across-sessions` | implemented semantic mapping | `kani` |
| `doc/source/eve/modular-assembly.rst::set-transform-persists-per-part` | implemented semantic mapping | `kani` |
| `doc/source/eve/generic-pp-effects.rst::identical-effects-group` | implemented semantic mapping | `kani` |
| `doc/source/eve/generic-pp-effects.rst::non-blendable-parameters-split-groups` | implemented semantic mapping | `kani` |
| `trinityal/tests/BitmapDimensions.cpp::BitmapDimensions::InitialSizeIsZero` | implemented semantic mapping | `kani` |
| `trinityal/tests/BitmapDimensions.cpp::BitmapDimensions::ReportsCorrect2DSizes` | implemented semantic mapping | `kani` |
| `trinityal/tests/BitmapDimensions.cpp::BitmapDimensions::ReportsCorrect3DSizes` | implemented semantic mapping | `kani` |
| `trinityal/tests/BitmapDimensions.cpp::BitmapDimensions::CalculatesTrueMipCountAndMipSizes` | implemented semantic mapping | `kani` |
| `trinityal/tests/BitmapDimensions.cpp::BitmapDimensions::GettingMipSizesForIncorrectMipReturnsZero` | implemented semantic mapping | `kani` |
| `trinityal/tests/BitmapDimensions.cpp::BitmapDimensions::CalculatesTrueMipCountAndMipSizesForNptTexture` | implemented semantic mapping | `kani` |
| `trinityal/tests/BitmapDimensions.cpp::BitmapDimensions::CalculatesCorrectMipSize` | implemented semantic mapping | `kani` |
| `trinityal/tests/ALResultTest.cpp::ALResult::DefaultResultIsSuccess` | implemented semantic mapping | `kani` |
| `trinityal/tests/ALResultTest.cpp::ALResult::CanCreateALResultFromHRESULT` | implemented semantic mapping | `kani` |
| `trinityal/tests/ALResultTest.cpp::ALResult::CanCopyResult` | implemented semantic mapping | `kani` |
| `trinityal/tests/ALResultTest.cpp::ALResult::CanCastALResultToHRESULT` | implemented semantic mapping | `kani` |
| `trinityal/tests/ALResultTest.cpp::ALResult::CanQueryResultCategory` | implemented semantic mapping | `kani` |
| `trinityal/tests/ALResultTest.cpp::ALResult::CanCheckIfResultIsSuccessful` | implemented semantic mapping | `kani` |
| `trinityal/tests/ALResultTest.cpp::ALResult::CanGetErrorMessage` | implemented semantic mapping | `kani` |
| `trinityal/tests/VertexDefinition.cpp::VertexDefinition::CanCompareVertexDefinitionItems` | implemented semantic mapping | `kani` |
| `trinityal/tests/VertexDefinition.cpp::VertexDefinition::CanAddItemsToVertexDefinition` | implemented semantic mapping | `kani` |
