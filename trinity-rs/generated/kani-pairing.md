# Generated Kani pairing report

Legacy declarations: **328**  
Rust semantic tests: **97**  
Mapped to discovered legacy declarations: **93**  
Genco-generated Kani harnesses: **97**

A generated Kani harness is emitted only for a real `#[trinity_test(..., verification = "kani")]` semantic body. Unimplemented legacy tests remain explicit compatibility obligations; the generator does not create placeholder proofs.

| Legacy contract | Rust semantic test | Verification | Kani wrapper |
|---|---|---|---|
| `trinity/tests/Python/tests/test_childhierarchy.py::TestChildHierarchy::test_initValues` | `trinity::child_init_values` | `kani` | generated |
| `trinity/tests/Python/tests/test_childhierarchy.py::TestChildHierarchy::test_ownerAssignment` | `trinity::owner_assignment` | `kani` | generated |
| `trinity/tests/Python/tests/test_childhierarchy.py::TestChildHierarchy::test_ownerRemoval` | `trinity::owner_removal` | `kani` | generated |
| `trinity/tests/Python/tests/test_childhierarchy.py::TestChildHierarchy::test_parentAssignment` | `trinity::parent_assignment` | `kani` | generated |
| `trinity/tests/Python/tests/test_childhierarchy.py::TestChildHierarchy::test_parentRemoval` | `trinity::parent_removal` | `kani` | generated |
| `trinity/tests/Python/tests/test_childhierarchy.py::TestChildHierarchy::test_transitiveOwnerAssignment` | `trinity::transitive_owner_assignment` | `kani` | generated |
| `trinity/tests/Python/tests/test_modular.py::TestModular::test_addAnimatedHull` | `trinity::add_animated_hull_is_not_instanced` | `kani` | generated |
| `trinity/tests/Python/tests/test_modular.py::TestModular::test_addStaticHull` | `trinity::add_static_hull_creates_one_static_mesh_group` | `kani` | generated |
| `trinity/tests/Python/tests/test_modular.py::TestModular::test_addTwoDifferentStaticHulls` | `trinity::add_two_different_static_hulls_make_two_groups` | `kani` | generated |
| `trinity/tests/Python/tests/test_modular.py::TestModular::test_addTwoStaticHulls` | `trinity::add_two_static_hulls_share_mesh_group` | `kani` | generated |
| `trinity/tests/Python/tests/test_modular.py::TestModular::test_addsLocators` | `trinity::adds_damage_locators` | `kani` | generated |
| `trinity/tests/Python/tests/test_modular.py::TestModular::test_audioHullContainerCarriesPartTag` | `trinity::audio_container_carries_part_tag` | `kani` | generated |
| `trinity/tests/Python/tests/test_modular.py::TestModular::test_audioHullCreatesPlacementContainer` | `trinity::audio_hull_creates_logical_container` | `kani` | generated |
| `trinity/tests/Python/tests/test_modular.py::TestModular::test_boundingSphereAccountsForTransforms` | `trinity::bounding_sphere_accounts_for_translation` | `kani` | generated |
| `trinity/tests/Python/tests/test_modular.py::TestModular::test_boundingSphereConcentricParts` | `trinity::bounding_sphere_concentric_parts_uses_largest` | `kani` | generated |
| `trinity/tests/Python/tests/test_modular.py::TestModular::test_boundingSphereEncapsulatesAllChildren` | `trinity::bounding_sphere_encapsulates_all_children` | `kani` | generated |
| `trinity/tests/Python/tests/test_modular.py::TestModular::test_boundingSphereScalesWithSetTransform` | `trinity::bounding_sphere_scales_with_transform` | `kani` | generated |
| `trinity/tests/Python/tests/test_modular.py::TestModular::test_boundingSphereUpdatesOnRemoval` | `trinity::bounding_sphere_updates_on_removal` | `kani` | generated |
| `trinity/tests/Python/tests/test_modular.py::TestModular::test_boundingSphereUpdatesOnSetTransform` | `trinity::bounding_sphere_updates_on_set_transform` | `kani` | generated |
| `trinity/tests/Python/tests/test_modular.py::TestModular::test_addsLocatorsMerged` | `trinity::damage_locators_merge_across_parts` | `kani` | generated |
| `trinity/tests/Python/tests/test_modular.py::TestModular::test_doubleRemoveRaises` | `trinity::double_remove_returns_key_error` | `kani` | generated |
| `trinity/tests/Python/tests/test_modular.py::TestModular::test_addHullEmptyFactionRaceUsesDefaults` | `trinity::empty_faction_and_race_use_seed_defaults` | `kani` | generated |
| `trinity/tests/Python/tests/test_modular.py::TestModular::test_getRotationAndScale` | `trinity::get_rotation_and_scale` | `kani` | generated |
| `trinity/tests/Python/tests/test_modular.py::TestModular::test_getTransformUnknownPartRaises` | `trinity::get_transform_unknown_part_returns_key_error` | `kani` | generated |
| `trinity/tests/Python/tests/test_modular.py::TestModular::test_getTransformReflectsSetTransform` | `trinity::getters_reflect_set_transform` | `kani` | generated |
| `trinity/tests/Python/tests/test_modular.py::TestModular::test_partIdRecycledAfterRemove` | `trinity::highest_part_id_is_recycled_after_remove` | `kani` | generated |
| `trinity/tests/Python/tests/test_modular.py::TestModular::test_locatorsInheritTransform` | `trinity::locators_inherit_transform` | `kani` | generated |
| `trinity/tests/Python/tests/test_modular.py::TestModifyExistingObject::test_modifyObjectWithoutPartDataSurvivesReopen` | `trinity::modifying_existing_object_survives_reopen` | `kani` | generated |
| `trinity/tests/Python/tests/test_modular.py::TestModular::test_moveLocators` | `trinity::moving_part_moves_locators` | `kani` | generated |
| `trinity/tests/Python/tests/test_modular.py::TestModular::test_partDataSurvivesModifierReopen` | `trinity::part_data_survives_modifier_reopen` | `kani` | generated |
| `trinity/tests/Python/tests/test_modular.py::TestModular::test_partIdsAreUnique` | `trinity::part_ids_are_unique` | `kani` | generated |
| `doc/source/eve/modular-assembly.rst::part-tags-stay-valid-across-sessions` | `trinity::part_tags_are_stable_and_unique` | `kani` | generated |
| `trinity/tests/Python/tests/test_modular.py::TestModular::test_removeAndReaddIntoSharedMesh` | `trinity::remove_and_readd_into_shared_mesh` | `kani` | generated |
| `trinity/tests/Python/tests/test_modular.py::TestModular::test_removeInvalidPartTagRaises` | `trinity::remove_invalid_tag_returns_key_error` | `kani` | generated |
| `trinity/tests/Python/tests/test_modular.py::TestModular::test_removeMiddleInstanceKeepsOthers` | `trinity::remove_middle_instance_keeps_others` | `kani` | generated |
| `trinity/tests/Python/tests/test_modular.py::TestModular::test_removeOneOfTwoInstances` | `trinity::remove_one_of_two_instances_keeps_one` | `kani` | generated |
| `trinity/tests/Python/tests/test_modular.py::TestModular::test_removeWrongID` | `trinity::remove_wrong_id_returns_key_error` | `kani` | generated |
| `trinity/tests/Python/tests/test_modular.py::TestModular::test_removeAudioHullRemovesContainer` | `trinity::removing_audio_hull_removes_logical_container` | `kani` | generated |
| `trinity/tests/Python/tests/test_modular.py::TestModular::test_removeFirstOfTwoDifferentHullsKeepsSecond` | `trinity::removing_first_different_hull_keeps_second_group` | `kani` | generated |
| `trinity/tests/Python/tests/test_modular.py::TestModular::test_setBoundingSphere` | `trinity::set_bounding_sphere_from_one_part` | `kani` | generated |
| `trinity/tests/Python/tests/test_modular.py::TestModular::test_setTransformRemovedPartRaises` | `trinity::set_transform_removed_part_returns_key_error` | `kani` | generated |
| `doc/source/eve/modular-assembly.rst::set-transform-persists-per-part` | `trinity::set_transform_updates_selected_part` | `kani` | generated |
| `trinity/tests/Python/tests/test_modular.py::TestModular::test_shapeEllipsoidEncapsulatesAllParts` | `trinity::shape_ellipsoid_center_tracks_parts` | `kani` | generated |
| `trinity/tests/Python/tests/test_modular.py::TestModular::test_shapeEllipsoidEmptyAfterRemovingAllParts` | `trinity::shape_ellipsoid_empty_after_removing_all_parts` | `kani` | generated |
| `trinity/tests/Python/tests/test_modular.py::TestModular::test_staticHullWithoutEmittersHasNoContainer` | `trinity::static_hull_without_emitters_has_no_container` | `kani` | generated |
| `trinity/tests/Python/tests/test_modular.py::TestModular::test_addHullUnknownHullReturnsInvalidTag` | `trinity::unknown_hull_returns_invalid_tag` | `kani` | generated |
| `doc/source/eve/generic-pp-effects.rst::identical-effects-group` | `trinity::blendable_parameters_do_not_split_groups` | `kani` | generated |
| `doc/source/eve/generic-pp-effects.rst::non-blendable-parameters-split-groups` | `trinity::non_blendable_parameters_split_groups` | `kani` | generated |
| `trinityal/tests/BitmapDimensions.cpp::BitmapDimensions::CalculatesCorrectMipSize` | `trinityal::calculates_correct_mip_size` | `kani` | generated |
| `trinityal/tests/BitmapDimensions.cpp::BitmapDimensions::CalculatesTrueMipCountAndMipSizes` | `trinityal::calculates_true_mip_count_and_mip_sizes` | `kani` | generated |
| `trinityal/tests/BitmapDimensions.cpp::BitmapDimensions::CalculatesTrueMipCountAndMipSizesForNptTexture` | `trinityal::calculates_true_mip_count_for_npt_texture` | `kani` | generated |
| `trinityal/tests/BitmapDimensions.cpp::BitmapDimensions::GettingMipSizesForIncorrectMipReturnsZero` | `trinityal::getting_mip_sizes_for_incorrect_mip_returns_zero` | `kani` | generated |
| `trinityal/tests/BitmapDimensions.cpp::BitmapDimensions::InitialSizeIsZero` | `trinityal::initial_size_is_zero` | `kani` | generated |
| `trinityal/tests/BitmapDimensions.cpp::BitmapDimensions::ReportsCorrect2DSizes` | `trinityal::reports_correct_2d_sizes` | `kani` | generated |
| `trinityal/tests/BitmapDimensions.cpp::BitmapDimensions::ReportsCorrect3DSizes` | `trinityal::reports_correct_3d_sizes` | `kani` | generated |
| `trinityal/tests/ALResultTest.cpp::ALResult::CanAssignResult` | `trinityal::can_assign_result` | `kani` | generated |
| `trinityal/tests/ALResultTest.cpp::ALResult::CanCastALResultToHRESULT` | `trinityal::can_cast_al_result_to_hresult` | `kani` | generated |
| `trinityal/tests/ALResultTest.cpp::ALResult::CanCheckIfResultIsSuccessful` | `trinityal::can_check_if_result_is_successful` | `kani` | generated |
| `trinityal/tests/ALResultTest.cpp::ALResult::CanCopyResult` | `trinityal::can_copy_result` | `kani` | generated |
| `trinityal/tests/ALResultTest.cpp::ALResult::CanCreateALResultFromHRESULT` | `trinityal::can_create_al_result_from_hresult` | `kani` | generated |
| `trinityal/tests/ALResultTest.cpp::ALResult::CanGetErrorMessage` | `trinityal::can_get_error_message` | `kani` | generated |
| `trinityal/tests/ALResultTest.cpp::ALResult::CanQueryResultCategory` | `trinityal::can_query_result_category` | `kani` | generated |
| `trinityal/tests/ALResultTest.cpp::ALResult::DefaultResultIsSuccess` | `trinityal::default_result_is_success` | `kani` | generated |
| `trinityal/tests/TextureSubresource.cpp::TextureSubresource::CanAdvanceMipForCompressedTexture` | `trinityal::can_advance_mip_for_compressed_texture` | `kani` | generated |
| `trinityal/tests/TextureSubresource.cpp::TextureSubresource::CanAdvanceMipForUncompressedTexture` | `trinityal::can_advance_mip_for_uncompressed_texture` | `kani` | generated |
| `trinityal/tests/TextureSubresource.cpp::TextureSubresource::CanClampTextureSubresourceTo2DTexture` | `trinityal::can_clamp_to_2d_texture` | `kani` | generated |
| `trinityal/tests/TextureSubresource.cpp::TextureSubresource::CanClampTextureSubresourceTo3DTexture` | `trinityal::can_clamp_to_3d_texture` | `kani` | generated |
| `trinityal/tests/TextureSubresource.cpp::TextureSubresource::CanClampTextureSubresourceToCubeTexture` | `trinityal::can_clamp_to_cube_texture` | `kani` | generated |
| `trinityal/tests/TextureSubresource.cpp::TextureSubresource::CroppingTextureSubresourcesWithDifferentFaceCountFails` | `trinityal::cropping_different_face_count_fails` | `kani` | generated |
| `trinityal/tests/TextureSubresource.cpp::TextureSubresource::CroppingFullTextureSubresourcesWithSameBitmapSucceeds` | `trinityal::cropping_full_subresources_with_same_bitmap_succeeds` | `kani` | generated |
| `trinityal/tests/TextureSubresource.cpp::TextureSubresource::TextureSubresourceDefaultConstructorCreatesFullResource` | `trinityal::default_constructor_creates_full_resource` | `kani` | generated |
| `trinityal/tests/TextureSubresource.cpp::TextureSubresource::DefaultTextureSubresourceCovers2DTexture` | `trinityal::default_subresource_covers_2d_texture` | `kani` | generated |
| `trinityal/tests/TextureSubresource.cpp::TextureSubresource::DifferentTextureSubresourcesAreNotEqual` | `trinityal::different_subresources_are_not_equal` | `kani` | generated |
| `trinityal/tests/TextureSubresource.cpp::TextureSubresource::TextureSubresourceFaceAndMipLevelConstructorCreatesFullFaceMipLevel` | `trinityal::face_and_mip_constructor_creates_full_face_mip` | `kani` | generated |
| `trinityal/tests/TextureSubresource.cpp::TextureSubresource::TextureSubresourceWithFacesDoesNotCoverCubeTexture` | `trinityal::faces_do_not_cover_cube_texture` | `kani` | generated |
| `trinityal/tests/TextureSubresource.cpp::TextureSubresource::IdenticalTextureSubresourcesAreEqual` | `trinityal::identical_subresources_are_equal` | `kani` | generated |
| `trinityal/tests/TextureSubresource.cpp::TextureSubresource::TextureSubresourceWithIncorrectFacesIsNotValid` | `trinityal::incorrect_faces_is_not_valid` | `kani` | generated |
| `trinityal/tests/TextureSubresource.cpp::TextureSubresource::TextureSubresourceWithIncorrectFrontAndBackIsNotValid` | `trinityal::incorrect_front_and_back_is_not_valid` | `kani` | generated |
| `trinityal/tests/TextureSubresource.cpp::TextureSubresource::TextureSubresourceWithIncorrectLeftAndRightIsNotValid` | `trinityal::incorrect_left_and_right_is_not_valid` | `kani` | generated |
| `trinityal/tests/TextureSubresource.cpp::TextureSubresource::TextureSubresourceWithIncorrectMipLevelsIsNotValid` | `trinityal::incorrect_mip_levels_is_not_valid` | `kani` | generated |
| `trinityal/tests/TextureSubresource.cpp::TextureSubresource::TextureSubresourceWithIncorrectTopAndBottomIsNotValid` | `trinityal::incorrect_top_and_bottom_is_not_valid` | `kani` | generated |
| `trinityal/tests/TextureSubresource.cpp::TextureSubresource::TextureSubresourceWithOffsetDoesNotCover2DTexture` | `trinityal::offset_does_not_cover_2d_texture` | `kani` | generated |
| `trinityal/tests/TextureSubresource.cpp::TextureSubresource::TextureSubresourceReportsCorrectDepth` | `trinityal::reports_correct_depth` | `kani` | generated |
| `trinityal/tests/TextureSubresource.cpp::TextureSubresource::TextureSubresourceReportsCorrectFaceCount` | `trinityal::reports_correct_face_count` | `kani` | generated |
| `trinityal/tests/TextureSubresource.cpp::TextureSubresource::TextureSubresourceReportsCorrectHeight` | `trinityal::reports_correct_height` | `kani` | generated |
| `trinityal/tests/TextureSubresource.cpp::TextureSubresource::TextureSubresourceReportsCorrectMipCount` | `trinityal::reports_correct_mip_count` | `kani` | generated |
| `trinityal/tests/TextureSubresource.cpp::TextureSubresource::TextureSubresourceReportsCorrectWidth` | `trinityal::reports_correct_width` | `kani` | generated |
| `trinityal/tests/TextureSubresource.cpp::TextureSubresource::TextureSubresourceWithSmallerMipmapsDoesNotCover2DTexture` | `trinityal::smaller_mipmaps_do_not_cover_2d_texture` | `kani` | generated |
| `trinityal/tests/TextureSubresource.cpp::TextureSubresource::SmallerTextureSubresourceDoesNotCover2DTexture` | `trinityal::smaller_subresource_does_not_cover_2d_texture` | `kani` | generated |
| `trinityal/tests/TextureSubresource.cpp::TextureSubresource::TextureSubresourceWithStartMipmapDoesNotCover2DTexture` | `trinityal::start_mipmap_does_not_cover_2d_texture` | `kani` | generated |
| `trinityal/tests/VertexDefinition.cpp::VertexDefinition::CanAddItemsToVertexDefinition` | `trinityal::can_add_items_to_vertex_definition` | `kani` | generated |
| `trinityal/tests/VertexDefinition.cpp::VertexDefinition::CanCompareVertexDefinitionItems` | `trinityal::can_compare_vertex_definition_items` | `kani` | generated |
| `trinityal/tests/VertexDefinition.cpp::VertexDefinition::CanCompareVertexDefinitions` | `trinityal::can_compare_vertex_definitions` | `kani` | generated |
| `trinityal/tests/VertexDefinition.cpp::VertexDefinition::CanMoveVertexDefinition` | `trinityal::can_move_vertex_definition` | `kani` | generated |
| `trinityal/tests/VertexDefinition.cpp::VertexDefinition::CanQueryVertexDefinitionItemSizeInBytes` | `trinityal::can_query_item_size_in_bytes` | `kani` | generated |
| `trinityal/tests/VertexDefinition.cpp::VertexDefinition::CanQueryVertexDefinitionItemSizeInMembers` | `trinityal::can_query_item_size_in_members` | `kani` | generated |
| `trinityal/tests/VertexDefinition.cpp::VertexDefinition::CanSearchForVertexDefinitionItems` | `trinityal::can_search_for_vertex_definition_items` | `kani` | generated |
