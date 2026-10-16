#![cfg(kani)]
#![allow(dead_code)]
#[path = "hierarchy.rs"]
mod __trinity_source_hierarchy_rs;
#[path = "modular.rs"]
mod __trinity_source_modular_rs;
#[path = "postprocess.rs"]
mod __trinity_source_postprocess_rs;
#[kani::proof]
fn trinity_kani_trinity_child_init_values() {
    __trinity_source_hierarchy_rs::__trinity_test_body_child_init_values();
}
#[kani::proof]
fn trinity_kani_trinity_owner_assignment() {
    __trinity_source_hierarchy_rs::__trinity_test_body_owner_assignment();
}
#[kani::proof]
fn trinity_kani_trinity_owner_removal() {
    __trinity_source_hierarchy_rs::__trinity_test_body_owner_removal();
}
#[kani::proof]
fn trinity_kani_trinity_parent_assignment() {
    __trinity_source_hierarchy_rs::__trinity_test_body_parent_assignment();
}
#[kani::proof]
fn trinity_kani_trinity_parent_removal() {
    __trinity_source_hierarchy_rs::__trinity_test_body_parent_removal();
}
#[kani::proof]
fn trinity_kani_trinity_transitive_owner_assignment() {
    __trinity_source_hierarchy_rs::__trinity_test_body_transitive_owner_assignment();
}
#[kani::proof]
fn trinity_kani_trinity_add_animated_hull_is_not_instanced() {
    __trinity_source_modular_rs::__trinity_test_body_add_animated_hull_is_not_instanced();
}
#[kani::proof]
fn trinity_kani_trinity_add_static_hull_creates_one_static_mesh_group() {
    __trinity_source_modular_rs::__trinity_test_body_add_static_hull_creates_one_static_mesh_group(
    );
}
#[kani::proof]
fn trinity_kani_trinity_add_two_different_static_hulls_make_two_groups() {
    __trinity_source_modular_rs::__trinity_test_body_add_two_different_static_hulls_make_two_groups(
    );
}
#[kani::proof]
fn trinity_kani_trinity_add_two_static_hulls_share_mesh_group() {
    __trinity_source_modular_rs::__trinity_test_body_add_two_static_hulls_share_mesh_group();
}
#[kani::proof]
fn trinity_kani_trinity_adds_damage_locators() {
    __trinity_source_modular_rs::__trinity_test_body_adds_damage_locators();
}
#[kani::proof]
fn trinity_kani_trinity_audio_container_carries_part_tag() {
    __trinity_source_modular_rs::__trinity_test_body_audio_container_carries_part_tag();
}
#[kani::proof]
fn trinity_kani_trinity_audio_hull_creates_logical_container() {
    __trinity_source_modular_rs::__trinity_test_body_audio_hull_creates_logical_container();
}
#[kani::proof]
fn trinity_kani_trinity_bounding_sphere_accounts_for_translation() {
    __trinity_source_modular_rs::__trinity_test_body_bounding_sphere_accounts_for_translation();
}
#[kani::proof]
fn trinity_kani_trinity_bounding_sphere_concentric_parts_uses_largest() {
    __trinity_source_modular_rs::__trinity_test_body_bounding_sphere_concentric_parts_uses_largest(
    );
}
#[kani::proof]
fn trinity_kani_trinity_bounding_sphere_encapsulates_all_children() {
    __trinity_source_modular_rs::__trinity_test_body_bounding_sphere_encapsulates_all_children();
}
#[kani::proof]
fn trinity_kani_trinity_bounding_sphere_scales_with_transform() {
    __trinity_source_modular_rs::__trinity_test_body_bounding_sphere_scales_with_transform();
}
#[kani::proof]
fn trinity_kani_trinity_bounding_sphere_updates_on_removal() {
    __trinity_source_modular_rs::__trinity_test_body_bounding_sphere_updates_on_removal();
}
#[kani::proof]
fn trinity_kani_trinity_bounding_sphere_updates_on_set_transform() {
    __trinity_source_modular_rs::__trinity_test_body_bounding_sphere_updates_on_set_transform();
}
#[kani::proof]
fn trinity_kani_trinity_damage_locators_merge_across_parts() {
    __trinity_source_modular_rs::__trinity_test_body_damage_locators_merge_across_parts();
}
#[kani::proof]
fn trinity_kani_trinity_double_remove_returns_key_error() {
    __trinity_source_modular_rs::__trinity_test_body_double_remove_returns_key_error();
}
#[kani::proof]
fn trinity_kani_trinity_empty_faction_and_race_use_seed_defaults() {
    __trinity_source_modular_rs::__trinity_test_body_empty_faction_and_race_use_seed_defaults();
}
#[kani::proof]
fn trinity_kani_trinity_get_rotation_and_scale() {
    __trinity_source_modular_rs::__trinity_test_body_get_rotation_and_scale();
}
#[kani::proof]
fn trinity_kani_trinity_get_transform_unknown_part_returns_key_error() {
    __trinity_source_modular_rs::__trinity_test_body_get_transform_unknown_part_returns_key_error();
}
#[kani::proof]
fn trinity_kani_trinity_getters_reflect_set_transform() {
    __trinity_source_modular_rs::__trinity_test_body_getters_reflect_set_transform();
}
#[kani::proof]
fn trinity_kani_trinity_highest_part_id_is_recycled_after_remove() {
    __trinity_source_modular_rs::__trinity_test_body_highest_part_id_is_recycled_after_remove();
}
#[kani::proof]
fn trinity_kani_trinity_locators_inherit_transform() {
    __trinity_source_modular_rs::__trinity_test_body_locators_inherit_transform();
}
#[kani::proof]
fn trinity_kani_trinity_modifying_existing_object_survives_reopen() {
    __trinity_source_modular_rs::__trinity_test_body_modifying_existing_object_survives_reopen();
}
#[kani::proof]
fn trinity_kani_trinity_moving_part_moves_locators() {
    __trinity_source_modular_rs::__trinity_test_body_moving_part_moves_locators();
}
#[kani::proof]
fn trinity_kani_trinity_part_data_survives_modifier_reopen() {
    __trinity_source_modular_rs::__trinity_test_body_part_data_survives_modifier_reopen();
}
#[kani::proof]
fn trinity_kani_trinity_part_ids_are_unique() {
    __trinity_source_modular_rs::__trinity_test_body_part_ids_are_unique();
}
#[kani::proof]
fn trinity_kani_trinity_part_tags_are_stable_and_unique() {
    __trinity_source_modular_rs::__trinity_test_body_part_tags_are_stable_and_unique();
}
#[kani::proof]
fn trinity_kani_trinity_remove_and_readd_into_shared_mesh() {
    __trinity_source_modular_rs::__trinity_test_body_remove_and_readd_into_shared_mesh();
}
#[kani::proof]
fn trinity_kani_trinity_remove_invalid_tag_returns_key_error() {
    __trinity_source_modular_rs::__trinity_test_body_remove_invalid_tag_returns_key_error();
}
#[kani::proof]
fn trinity_kani_trinity_remove_middle_instance_keeps_others() {
    __trinity_source_modular_rs::__trinity_test_body_remove_middle_instance_keeps_others();
}
#[kani::proof]
fn trinity_kani_trinity_remove_one_of_two_instances_keeps_one() {
    __trinity_source_modular_rs::__trinity_test_body_remove_one_of_two_instances_keeps_one();
}
#[kani::proof]
fn trinity_kani_trinity_remove_wrong_id_returns_key_error() {
    __trinity_source_modular_rs::__trinity_test_body_remove_wrong_id_returns_key_error();
}
#[kani::proof]
fn trinity_kani_trinity_removing_audio_hull_removes_logical_container() {
    __trinity_source_modular_rs::__trinity_test_body_removing_audio_hull_removes_logical_container(
    );
}
#[kani::proof]
fn trinity_kani_trinity_removing_first_different_hull_keeps_second_group() {
    __trinity_source_modular_rs::__trinity_test_body_removing_first_different_hull_keeps_second_group();
}
#[kani::proof]
fn trinity_kani_trinity_set_bounding_sphere_from_one_part() {
    __trinity_source_modular_rs::__trinity_test_body_set_bounding_sphere_from_one_part();
}
#[kani::proof]
fn trinity_kani_trinity_set_transform_removed_part_returns_key_error() {
    __trinity_source_modular_rs::__trinity_test_body_set_transform_removed_part_returns_key_error();
}
#[kani::proof]
fn trinity_kani_trinity_set_transform_updates_selected_part() {
    __trinity_source_modular_rs::__trinity_test_body_set_transform_updates_selected_part();
}
#[kani::proof]
fn trinity_kani_trinity_shape_ellipsoid_center_tracks_parts() {
    __trinity_source_modular_rs::__trinity_test_body_shape_ellipsoid_center_tracks_parts();
}
#[kani::proof]
fn trinity_kani_trinity_shape_ellipsoid_empty_after_removing_all_parts() {
    __trinity_source_modular_rs::__trinity_test_body_shape_ellipsoid_empty_after_removing_all_parts(
    );
}
#[kani::proof]
fn trinity_kani_trinity_static_hull_without_emitters_has_no_container() {
    __trinity_source_modular_rs::__trinity_test_body_static_hull_without_emitters_has_no_container(
    );
}
#[kani::proof]
fn trinity_kani_trinity_unknown_hull_returns_invalid_tag() {
    __trinity_source_modular_rs::__trinity_test_body_unknown_hull_returns_invalid_tag();
}
#[kani::proof]
fn trinity_kani_trinity_blendable_parameters_do_not_split_groups() {
    __trinity_source_postprocess_rs::__trinity_test_body_blendable_parameters_do_not_split_groups();
}
#[kani::proof]
fn trinity_kani_trinity_non_blendable_parameters_split_groups() {
    __trinity_source_postprocess_rs::__trinity_test_body_non_blendable_parameters_split_groups();
}
