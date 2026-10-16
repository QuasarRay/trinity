#![cfg(kani)]
#![allow(dead_code)]#[path = "bitmap.rs"]
mod __trinity_source_bitmap_rs;#[path = "result.rs"]
mod __trinity_source_result_rs;#[path = "subresource.rs"]
mod __trinity_source_subresource_rs;#[path = "vertex.rs"]
mod __trinity_source_vertex_rs;#[kani::proof]
fn trinity_kani_trinityal_calculates_correct_mip_size() {
    __trinity_source_bitmap_rs::__trinity_test_body_calculates_correct_mip_size();
}#[kani::proof]
fn trinity_kani_trinityal_calculates_true_mip_count_and_mip_sizes() {
    __trinity_source_bitmap_rs::__trinity_test_body_calculates_true_mip_count_and_mip_sizes();
}#[kani::proof]
fn trinity_kani_trinityal_calculates_true_mip_count_for_npt_texture() {
    __trinity_source_bitmap_rs::__trinity_test_body_calculates_true_mip_count_for_npt_texture();
}#[kani::proof]
fn trinity_kani_trinityal_getting_mip_sizes_for_incorrect_mip_returns_zero() {
    __trinity_source_bitmap_rs::__trinity_test_body_getting_mip_sizes_for_incorrect_mip_returns_zero();
}#[kani::proof]
fn trinity_kani_trinityal_initial_size_is_zero() {
    __trinity_source_bitmap_rs::__trinity_test_body_initial_size_is_zero();
}#[kani::proof]
fn trinity_kani_trinityal_reports_correct_2d_sizes() {
    __trinity_source_bitmap_rs::__trinity_test_body_reports_correct_2d_sizes();
}#[kani::proof]
fn trinity_kani_trinityal_reports_correct_3d_sizes() {
    __trinity_source_bitmap_rs::__trinity_test_body_reports_correct_3d_sizes();
}#[kani::proof]
fn trinity_kani_trinityal_can_assign_result() {
    __trinity_source_result_rs::__trinity_test_body_can_assign_result();
}#[kani::proof]
fn trinity_kani_trinityal_can_cast_al_result_to_hresult() {
    __trinity_source_result_rs::__trinity_test_body_can_cast_al_result_to_hresult();
}#[kani::proof]
fn trinity_kani_trinityal_can_check_if_result_is_successful() {
    __trinity_source_result_rs::__trinity_test_body_can_check_if_result_is_successful();
}#[kani::proof]
fn trinity_kani_trinityal_can_copy_result() {
    __trinity_source_result_rs::__trinity_test_body_can_copy_result();
}#[kani::proof]
fn trinity_kani_trinityal_can_create_al_result_from_hresult() {
    __trinity_source_result_rs::__trinity_test_body_can_create_al_result_from_hresult();
}#[kani::proof]
fn trinity_kani_trinityal_can_get_error_message() {
    __trinity_source_result_rs::__trinity_test_body_can_get_error_message();
}#[kani::proof]
fn trinity_kani_trinityal_can_query_result_category() {
    __trinity_source_result_rs::__trinity_test_body_can_query_result_category();
}#[kani::proof]
fn trinity_kani_trinityal_default_result_is_success() {
    __trinity_source_result_rs::__trinity_test_body_default_result_is_success();
}#[kani::proof]
fn trinity_kani_trinityal_can_advance_mip_for_compressed_texture() {
    __trinity_source_subresource_rs::__trinity_test_body_can_advance_mip_for_compressed_texture();
}#[kani::proof]
fn trinity_kani_trinityal_can_advance_mip_for_uncompressed_texture() {
    __trinity_source_subresource_rs::__trinity_test_body_can_advance_mip_for_uncompressed_texture();
}#[kani::proof]
fn trinity_kani_trinityal_can_clamp_to_2d_texture() {
    __trinity_source_subresource_rs::__trinity_test_body_can_clamp_to_2d_texture();
}#[kani::proof]
fn trinity_kani_trinityal_can_clamp_to_3d_texture() {
    __trinity_source_subresource_rs::__trinity_test_body_can_clamp_to_3d_texture();
}#[kani::proof]
fn trinity_kani_trinityal_can_clamp_to_cube_texture() {
    __trinity_source_subresource_rs::__trinity_test_body_can_clamp_to_cube_texture();
}#[kani::proof]
fn trinity_kani_trinityal_cropping_different_face_count_fails() {
    __trinity_source_subresource_rs::__trinity_test_body_cropping_different_face_count_fails();
}#[kani::proof]
fn trinity_kani_trinityal_cropping_full_subresources_with_same_bitmap_succeeds() {
    __trinity_source_subresource_rs::__trinity_test_body_cropping_full_subresources_with_same_bitmap_succeeds();
}#[kani::proof]
fn trinity_kani_trinityal_default_constructor_creates_full_resource() {
    __trinity_source_subresource_rs::__trinity_test_body_default_constructor_creates_full_resource();
}#[kani::proof]
fn trinity_kani_trinityal_default_subresource_covers_2d_texture() {
    __trinity_source_subresource_rs::__trinity_test_body_default_subresource_covers_2d_texture();
}#[kani::proof]
fn trinity_kani_trinityal_different_subresources_are_not_equal() {
    __trinity_source_subresource_rs::__trinity_test_body_different_subresources_are_not_equal();
}#[kani::proof]
fn trinity_kani_trinityal_face_and_mip_constructor_creates_full_face_mip() {
    __trinity_source_subresource_rs::__trinity_test_body_face_and_mip_constructor_creates_full_face_mip();
}#[kani::proof]
fn trinity_kani_trinityal_faces_do_not_cover_cube_texture() {
    __trinity_source_subresource_rs::__trinity_test_body_faces_do_not_cover_cube_texture();
}#[kani::proof]
fn trinity_kani_trinityal_identical_subresources_are_equal() {
    __trinity_source_subresource_rs::__trinity_test_body_identical_subresources_are_equal();
}#[kani::proof]
fn trinity_kani_trinityal_incorrect_faces_is_not_valid() {
    __trinity_source_subresource_rs::__trinity_test_body_incorrect_faces_is_not_valid();
}#[kani::proof]
fn trinity_kani_trinityal_incorrect_front_and_back_is_not_valid() {
    __trinity_source_subresource_rs::__trinity_test_body_incorrect_front_and_back_is_not_valid();
}#[kani::proof]
fn trinity_kani_trinityal_incorrect_left_and_right_is_not_valid() {
    __trinity_source_subresource_rs::__trinity_test_body_incorrect_left_and_right_is_not_valid();
}#[kani::proof]
fn trinity_kani_trinityal_incorrect_mip_levels_is_not_valid() {
    __trinity_source_subresource_rs::__trinity_test_body_incorrect_mip_levels_is_not_valid();
}#[kani::proof]
fn trinity_kani_trinityal_incorrect_top_and_bottom_is_not_valid() {
    __trinity_source_subresource_rs::__trinity_test_body_incorrect_top_and_bottom_is_not_valid();
}#[kani::proof]
fn trinity_kani_trinityal_offset_does_not_cover_2d_texture() {
    __trinity_source_subresource_rs::__trinity_test_body_offset_does_not_cover_2d_texture();
}#[kani::proof]
fn trinity_kani_trinityal_reports_correct_depth() {
    __trinity_source_subresource_rs::__trinity_test_body_reports_correct_depth();
}#[kani::proof]
fn trinity_kani_trinityal_reports_correct_face_count() {
    __trinity_source_subresource_rs::__trinity_test_body_reports_correct_face_count();
}#[kani::proof]
fn trinity_kani_trinityal_reports_correct_height() {
    __trinity_source_subresource_rs::__trinity_test_body_reports_correct_height();
}#[kani::proof]
fn trinity_kani_trinityal_reports_correct_mip_count() {
    __trinity_source_subresource_rs::__trinity_test_body_reports_correct_mip_count();
}#[kani::proof]
fn trinity_kani_trinityal_reports_correct_width() {
    __trinity_source_subresource_rs::__trinity_test_body_reports_correct_width();
}#[kani::proof]
fn trinity_kani_trinityal_smaller_mipmaps_do_not_cover_2d_texture() {
    __trinity_source_subresource_rs::__trinity_test_body_smaller_mipmaps_do_not_cover_2d_texture();
}#[kani::proof]
fn trinity_kani_trinityal_smaller_subresource_does_not_cover_2d_texture() {
    __trinity_source_subresource_rs::__trinity_test_body_smaller_subresource_does_not_cover_2d_texture();
}#[kani::proof]
fn trinity_kani_trinityal_start_mipmap_does_not_cover_2d_texture() {
    __trinity_source_subresource_rs::__trinity_test_body_start_mipmap_does_not_cover_2d_texture();
}#[kani::proof]
fn trinity_kani_trinityal_can_add_items_to_vertex_definition() {
    __trinity_source_vertex_rs::__trinity_test_body_can_add_items_to_vertex_definition();
}#[kani::proof]
fn trinity_kani_trinityal_can_compare_vertex_definition_items() {
    __trinity_source_vertex_rs::__trinity_test_body_can_compare_vertex_definition_items();
}#[kani::proof]
fn trinity_kani_trinityal_can_compare_vertex_definitions() {
    __trinity_source_vertex_rs::__trinity_test_body_can_compare_vertex_definitions();
}#[kani::proof]
fn trinity_kani_trinityal_can_move_vertex_definition() {
    __trinity_source_vertex_rs::__trinity_test_body_can_move_vertex_definition();
}#[kani::proof]
fn trinity_kani_trinityal_can_query_item_size_in_bytes() {
    __trinity_source_vertex_rs::__trinity_test_body_can_query_item_size_in_bytes();
}#[kani::proof]
fn trinity_kani_trinityal_can_query_item_size_in_members() {
    __trinity_source_vertex_rs::__trinity_test_body_can_query_item_size_in_members();
}#[kani::proof]
fn trinity_kani_trinityal_can_search_for_vertex_definition_items() {
    __trinity_source_vertex_rs::__trinity_test_body_can_search_for_vertex_definition_items();
}
