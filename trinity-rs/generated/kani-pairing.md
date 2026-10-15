# Bootstrap Kani pairing report

Rust semantic tests: **26**  
Checked-in generated Kani wrappers: **26**

This bootstrap report is replaced by `trinity-codegen` when run inside the full Trinity repository checkout. The canonical generator merges these mappings with every discovered original GoogleTest/Python test and Doxygen symbol.

| Legacy contract | Rust semantic test | Verification |
|---|---|---|
| `trinity/tests/Python/tests/test_childhierarchy.py::TestChildHierarchy::test_initValues` | `trinity::child_init_values` | `kani` |
| `trinity/tests/Python/tests/test_childhierarchy.py::TestChildHierarchy::test_ownerAssignment` | `trinity::owner_assignment` | `kani` |
| `trinity/tests/Python/tests/test_childhierarchy.py::TestChildHierarchy::test_transitiveOwnerAssignment` | `trinity::transitive_owner_assignment` | `kani` |
| `trinity/tests/Python/tests/test_childhierarchy.py::TestChildHierarchy::test_parentAssignment` | `trinity::parent_assignment` | `kani` |
| `trinity/tests/Python/tests/test_childhierarchy.py::TestChildHierarchy::test_ownerRemoval` | `trinity::owner_removal` | `kani` |
| `trinity/tests/Python/tests/test_childhierarchy.py::TestChildHierarchy::test_parentRemoval` | `trinity::parent_removal` | `kani` |
| `doc/source/eve/modular-assembly.rst::part-tags-stay-valid-across-sessions` | `trinity::part_tags_are_stable_and_unique` | `kani` |
| `doc/source/eve/modular-assembly.rst::set-transform-persists-per-part` | `trinity::set_transform_updates_selected_part` | `kani` |
| `doc/source/eve/generic-pp-effects.rst::identical-effects-group` | `trinity::blendable_parameters_do_not_split_groups` | `kani` |
| `doc/source/eve/generic-pp-effects.rst::non-blendable-parameters-split-groups` | `trinity::non_blendable_parameters_split_groups` | `kani` |
| `trinityal/tests/BitmapDimensions.cpp::BitmapDimensions::InitialSizeIsZero` | `trinityal::initial_size_is_zero` | `kani` |
| `trinityal/tests/BitmapDimensions.cpp::BitmapDimensions::ReportsCorrect2DSizes` | `trinityal::reports_correct_2d_sizes` | `kani` |
| `trinityal/tests/BitmapDimensions.cpp::BitmapDimensions::ReportsCorrect3DSizes` | `trinityal::reports_correct_3d_sizes` | `kani` |
| `trinityal/tests/BitmapDimensions.cpp::BitmapDimensions::CalculatesTrueMipCountAndMipSizes` | `trinityal::calculates_true_mip_count_and_mip_sizes` | `kani` |
| `trinityal/tests/BitmapDimensions.cpp::BitmapDimensions::GettingMipSizesForIncorrectMipReturnsZero` | `trinityal::getting_mip_sizes_for_incorrect_mip_returns_zero` | `kani` |
| `trinityal/tests/BitmapDimensions.cpp::BitmapDimensions::CalculatesTrueMipCountAndMipSizesForNptTexture` | `trinityal::calculates_true_mip_count_for_npt_texture` | `kani` |
| `trinityal/tests/BitmapDimensions.cpp::BitmapDimensions::CalculatesCorrectMipSize` | `trinityal::calculates_correct_mip_size` | `kani` |
| `trinityal/tests/ALResultTest.cpp::ALResult::DefaultResultIsSuccess` | `trinityal::default_result_is_success` | `kani` |
| `trinityal/tests/ALResultTest.cpp::ALResult::CanCreateALResultFromHRESULT` | `trinityal::can_create_al_result_from_hresult` | `kani` |
| `trinityal/tests/ALResultTest.cpp::ALResult::CanCopyResult` | `trinityal::can_copy_result` | `kani` |
| `trinityal/tests/ALResultTest.cpp::ALResult::CanCastALResultToHRESULT` | `trinityal::can_cast_al_result_to_hresult` | `kani` |
| `trinityal/tests/ALResultTest.cpp::ALResult::CanQueryResultCategory` | `trinityal::can_query_result_category` | `kani` |
| `trinityal/tests/ALResultTest.cpp::ALResult::CanCheckIfResultIsSuccessful` | `trinityal::can_check_if_result_is_successful` | `kani` |
| `trinityal/tests/ALResultTest.cpp::ALResult::CanGetErrorMessage` | `trinityal::can_get_error_message` | `kani` |
| `trinityal/tests/VertexDefinition.cpp::VertexDefinition::CanCompareVertexDefinitionItems` | `trinityal::can_compare_vertex_definition_items` | `kani` |
| `trinityal/tests/VertexDefinition.cpp::VertexDefinition::CanAddItemsToVertexDefinition` | `trinityal::can_add_items_to_vertex_definition` | `kani` |
