use trinity_macros::trinity_test;
use trinityal::{
    PixelFormat, TextureType, Tr2BitmapDimensions, Tr2TextureSubresource, advance_mip, crop,
};

fn dimensions(
    texture_type: TextureType,
    width: u32,
    height: u32,
    depth: u32,
    mip_count: u32,
) -> Tr2BitmapDimensions {
    Tr2BitmapDimensions {
        texture_type,
        width,
        height,
        volume_depth: depth,
        mip_count,
        array_size: match texture_type {
            TextureType::Cube => 6,
            TextureType::Invalid => 0,
            TextureType::Texture2D | TextureType::Texture3D => 1,
        },
        ..Default::default()
    }
}

#[trinity_test(
    legacy = "trinityal/tests/TextureSubresource.cpp::TextureSubresource::TextureSubresourceDefaultConstructorCreatesFullResource",
    verification = "kani"
)]
fn default_constructor_creates_full_resource() {
    let sub = Tr2TextureSubresource::default();
    assert_eq!(0, sub.start_face);
    assert_eq!(u32::MAX, sub.end_face);
    assert_eq!(0, sub.start_mip_level);
    assert_eq!(u32::MAX, sub.end_mip_level);
    assert!(!sub.has_box());
    assert_eq!(u32::MAX, sub.box_.left);
    assert_eq!(u32::MAX, sub.box_.top);
    assert_eq!(u32::MAX, sub.box_.front);
    assert_eq!(u32::MAX, sub.box_.right);
    assert_eq!(u32::MAX, sub.box_.bottom);
    assert_eq!(u32::MAX, sub.box_.back);
    assert!(sub.is_valid());
}

#[trinity_test(
    legacy = "trinityal/tests/TextureSubresource.cpp::TextureSubresource::TextureSubresourceFaceAndMipLevelConstructorCreatesFullFaceMipLevel",
    verification = "kani"
)]
fn face_and_mip_constructor_creates_full_face_mip() {
    let sub = Tr2TextureSubresource::face_mip(4, 2);
    assert_eq!((4, 5), (sub.start_face, sub.end_face));
    assert_eq!((2, 3), (sub.start_mip_level, sub.end_mip_level));
    assert!(!sub.has_box());
    assert!(sub.is_valid());
}

#[trinity_test(
    legacy = "trinityal/tests/TextureSubresource.cpp::TextureSubresource::CanClampTextureSubresourceTo2DTexture",
    verification = "kani"
)]
fn can_clamp_to_2d_texture() {
    let mut sub = Tr2TextureSubresource::default();
    sub.clamp_to_texture(dimensions(TextureType::Texture2D, 26, 71, 0, 3));
    assert_eq!((0, 1), (sub.start_face, sub.end_face));
    assert_eq!((0, 3), (sub.start_mip_level, sub.end_mip_level));
    assert_eq!((0, 0, 0), (sub.box_.left, sub.box_.top, sub.box_.front));
    assert_eq!((26, 71, 1), (sub.box_.right, sub.box_.bottom, sub.box_.back));
}

#[trinity_test(
    legacy = "trinityal/tests/TextureSubresource.cpp::TextureSubresource::CanClampTextureSubresourceToCubeTexture",
    verification = "kani"
)]
fn can_clamp_to_cube_texture() {
    let mut sub = Tr2TextureSubresource::default();
    sub.clamp_to_texture(dimensions(TextureType::Cube, 13, 13, 0, 2));
    assert_eq!((0, 6), (sub.start_face, sub.end_face));
    assert_eq!((0, 2), (sub.start_mip_level, sub.end_mip_level));
    assert_eq!((0, 0, 0), (sub.box_.left, sub.box_.top, sub.box_.front));
    assert_eq!((13, 13, 1), (sub.box_.right, sub.box_.bottom, sub.box_.back));
}

#[trinity_test(
    legacy = "trinityal/tests/TextureSubresource.cpp::TextureSubresource::CanClampTextureSubresourceTo3DTexture",
    verification = "kani"
)]
fn can_clamp_to_3d_texture() {
    let mut sub = Tr2TextureSubresource::default();
    sub.clamp_to_texture(dimensions(TextureType::Texture3D, 26, 71, 15, 3));
    assert_eq!((0, 1), (sub.start_face, sub.end_face));
    assert_eq!((0, 3), (sub.start_mip_level, sub.end_mip_level));
    assert_eq!((0, 0, 0), (sub.box_.left, sub.box_.top, sub.box_.front));
    assert_eq!((26, 71, 15), (sub.box_.right, sub.box_.bottom, sub.box_.back));
}

#[trinity_test(
    legacy = "trinityal/tests/TextureSubresource.cpp::TextureSubresource::DefaultTextureSubresourceCovers2DTexture",
    verification = "kani"
)]
fn default_subresource_covers_2d_texture() {
    let sub = Tr2TextureSubresource::default();
    assert!(sub.is_subresource_full(dimensions(
        TextureType::Texture2D,
        26,
        71,
        0,
        3
    )));
}

#[trinity_test(
    legacy = "trinityal/tests/TextureSubresource.cpp::TextureSubresource::SmallerTextureSubresourceDoesNotCover2DTexture",
    verification = "kani"
)]
fn smaller_subresource_does_not_cover_2d_texture() {
    let mut sub = Tr2TextureSubresource::default();
    sub.box_.right = 21;
    assert!(!sub.is_subresource_full(dimensions(
        TextureType::Texture2D,
        26,
        71,
        0,
        3
    )));
}

#[trinity_test(
    legacy = "trinityal/tests/TextureSubresource.cpp::TextureSubresource::TextureSubresourceWithStartMipmapDoesNotCover2DTexture",
    verification = "kani"
)]
fn start_mipmap_does_not_cover_2d_texture() {
    let mut sub = Tr2TextureSubresource::default();
    sub.start_mip_level = 1;
    assert!(!sub.is_subresource_full(dimensions(
        TextureType::Texture2D,
        26,
        71,
        0,
        3
    )));
}

#[trinity_test(
    legacy = "trinityal/tests/TextureSubresource.cpp::TextureSubresource::TextureSubresourceWithSmallerMipmapsDoesNotCover2DTexture",
    verification = "kani"
)]
fn smaller_mipmaps_do_not_cover_2d_texture() {
    let mut sub = Tr2TextureSubresource::default();
    sub.end_mip_level = 1;
    assert!(!sub.is_subresource_full(dimensions(
        TextureType::Texture2D,
        26,
        71,
        0,
        3
    )));
}

#[trinity_test(
    legacy = "trinityal/tests/TextureSubresource.cpp::TextureSubresource::TextureSubresourceWithOffsetDoesNotCover2DTexture",
    verification = "kani"
)]
fn offset_does_not_cover_2d_texture() {
    let mut sub = Tr2TextureSubresource::default();
    sub.box_.top = 3;
    assert!(!sub.is_subresource_full(dimensions(
        TextureType::Texture2D,
        26,
        71,
        0,
        3
    )));
}

#[trinity_test(
    legacy = "trinityal/tests/TextureSubresource.cpp::TextureSubresource::TextureSubresourceWithFacesDoesNotCoverCubeTexture",
    verification = "kani"
)]
fn faces_do_not_cover_cube_texture() {
    let mut sub = Tr2TextureSubresource::default();
    sub.start_face = 3;
    assert!(!sub.is_subresource_full(dimensions(
        TextureType::Cube,
        26,
        26,
        0,
        0
    )));
}

#[trinity_test(
    legacy = "trinityal/tests/TextureSubresource.cpp::TextureSubresource::IdenticalTextureSubresourcesAreEqual",
    verification = "kani"
)]
fn identical_subresources_are_equal() {
    let mut first = Tr2TextureSubresource::default();
    first.start_face = 3;
    first.box_.left = 3;
    first.box_.right = 5;
    first.start_mip_level = 6;
    let second = first;
    assert_eq!(first, second);
}

#[trinity_test(
    legacy = "trinityal/tests/TextureSubresource.cpp::TextureSubresource::DifferentTextureSubresourcesAreNotEqual",
    verification = "kani"
)]
fn different_subresources_are_not_equal() {
    let mut first = Tr2TextureSubresource::default();
    first.start_face = 3;
    first.box_.left = 3;
    first.box_.right = 5;
    first.start_mip_level = 6;
    let mut second = first;
    second.box_.top = 12;
    assert_ne!(first, second);
}

#[trinity_test(
    legacy = "trinityal/tests/TextureSubresource.cpp::TextureSubresource::TextureSubresourceWithIncorrectLeftAndRightIsNotValid",
    verification = "kani"
)]
fn incorrect_left_and_right_is_not_valid() {
    let mut sub = Tr2TextureSubresource::default();
    sub.box_.left = 5;
    sub.box_.right = 3;
    assert!(!sub.is_valid());
}

#[trinity_test(
    legacy = "trinityal/tests/TextureSubresource.cpp::TextureSubresource::TextureSubresourceWithIncorrectTopAndBottomIsNotValid",
    verification = "kani"
)]
fn incorrect_top_and_bottom_is_not_valid() {
    let mut sub = Tr2TextureSubresource::default();
    sub.box_.top = 15;
    sub.box_.bottom = 15;
    assert!(!sub.is_valid());
}

#[trinity_test(
    legacy = "trinityal/tests/TextureSubresource.cpp::TextureSubresource::TextureSubresourceWithIncorrectFrontAndBackIsNotValid",
    verification = "kani"
)]
fn incorrect_front_and_back_is_not_valid() {
    let mut sub = Tr2TextureSubresource::default();
    sub.box_.front = 25;
    sub.box_.back = 21;
    assert!(!sub.is_valid());
}

#[trinity_test(
    legacy = "trinityal/tests/TextureSubresource.cpp::TextureSubresource::TextureSubresourceWithIncorrectMipLevelsIsNotValid",
    verification = "kani"
)]
fn incorrect_mip_levels_is_not_valid() {
    let mut sub = Tr2TextureSubresource::default();
    sub.start_mip_level = 2;
    sub.end_mip_level = 1;
    assert!(!sub.is_valid());
}

#[trinity_test(
    legacy = "trinityal/tests/TextureSubresource.cpp::TextureSubresource::TextureSubresourceWithIncorrectFacesIsNotValid",
    verification = "kani"
)]
fn incorrect_faces_is_not_valid() {
    let mut sub = Tr2TextureSubresource::default();
    sub.start_face = 1;
    sub.end_face = 0;
    assert!(!sub.is_valid());
}

#[trinity_test(
    legacy = "trinityal/tests/TextureSubresource.cpp::TextureSubresource::TextureSubresourceReportsCorrectWidth",
    verification = "kani"
)]
fn reports_correct_width() {
    let mut sub = Tr2TextureSubresource::default();
    sub.box_.left = 5;
    sub.box_.right = 27;
    assert_eq!(22, sub.get_width());
}

#[trinity_test(
    legacy = "trinityal/tests/TextureSubresource.cpp::TextureSubresource::TextureSubresourceReportsCorrectHeight",
    verification = "kani"
)]
fn reports_correct_height() {
    let mut sub = Tr2TextureSubresource::default();
    sub.box_.top = 5;
    sub.box_.bottom = 27;
    assert_eq!(22, sub.get_height());
}

#[trinity_test(
    legacy = "trinityal/tests/TextureSubresource.cpp::TextureSubresource::TextureSubresourceReportsCorrectDepth",
    verification = "kani"
)]
fn reports_correct_depth() {
    let mut sub = Tr2TextureSubresource::default();
    sub.box_.front = 5;
    sub.box_.back = 27;
    assert_eq!(22, sub.get_depth());
}

#[trinity_test(
    legacy = "trinityal/tests/TextureSubresource.cpp::TextureSubresource::TextureSubresourceReportsCorrectMipCount",
    verification = "kani"
)]
fn reports_correct_mip_count() {
    let mut sub = Tr2TextureSubresource::default();
    sub.start_mip_level = 2;
    sub.end_mip_level = 7;
    assert_eq!(5, sub.get_mip_count());
}

#[trinity_test(
    legacy = "trinityal/tests/TextureSubresource.cpp::TextureSubresource::TextureSubresourceReportsCorrectFaceCount",
    verification = "kani"
)]
fn reports_correct_face_count() {
    let mut sub = Tr2TextureSubresource::default();
    sub.start_face = 1;
    sub.end_face = 3;
    assert_eq!(2, sub.get_face_count());
}

#[trinity_test(
    legacy = "trinityal/tests/TextureSubresource.cpp::TextureSubresource::CroppingTextureSubresourcesWithDifferentFaceCountFails",
    verification = "kani"
)]
fn cropping_different_face_count_fails() {
    let bitmap = dimensions(TextureType::Cube, 26, 26, 0, 0);
    let mut source = Tr2TextureSubresource::default();
    source.start_face = 1;
    source.end_face = 3;
    let mut destination = Tr2TextureSubresource::default();
    destination.start_face = 1;
    destination.end_face = 2;
    assert!(!crop(&mut source, bitmap, &mut destination, bitmap));
}

#[trinity_test(
    legacy = "trinityal/tests/TextureSubresource.cpp::TextureSubresource::CroppingFullTextureSubresourcesWithSameBitmapSucceeds",
    verification = "kani"
)]
fn cropping_full_subresources_with_same_bitmap_succeeds() {
    let bitmap = dimensions(TextureType::Texture2D, 32, 16, 0, 0);
    let mut source = Tr2TextureSubresource::default();
    let mut destination = Tr2TextureSubresource::default();
    assert!(crop(&mut source, bitmap, &mut destination, bitmap));
    assert_eq!((0, 6), (source.start_mip_level, source.end_mip_level));
    assert_eq!((0, 0, 0), (source.box_.left, source.box_.top, source.box_.front));
    assert_eq!((32, 16, 1), (source.box_.right, source.box_.bottom, source.box_.back));
    assert_eq!(source, destination);
}

#[trinity_test(
    legacy = "trinityal/tests/TextureSubresource.cpp::TextureSubresource::CanAdvanceMipForUncompressedTexture",
    verification = "kani"
)]
fn can_advance_mip_for_uncompressed_texture() {
    let mut sub = Tr2TextureSubresource::default();
    sub.box_.left = 4;
    sub.box_.right = 24;
    sub.box_.top = 7;
    sub.box_.bottom = 10;
    let mut bitmap = dimensions(TextureType::Texture2D, 32, 16, 0, 0);
    bitmap.format = PixelFormat::B8G8R8A8Typeless;

    advance_mip(&mut sub, bitmap, 1);
    assert_eq!((2, 3, 12, 5), (sub.box_.left, sub.box_.top, sub.box_.right, sub.box_.bottom));

    advance_mip(&mut sub, bitmap, 2);
    assert_eq!((1, 1, 6, 2), (sub.box_.left, sub.box_.top, sub.box_.right, sub.box_.bottom));
}

#[trinity_test(
    legacy = "trinityal/tests/TextureSubresource.cpp::TextureSubresource::CanAdvanceMipForCompressedTexture",
    verification = "kani"
)]
fn can_advance_mip_for_compressed_texture() {
    let mut sub = Tr2TextureSubresource::default();
    sub.box_.left = 4;
    sub.box_.right = 24;
    sub.box_.top = 7;
    sub.box_.bottom = 10;
    let mut bitmap = dimensions(TextureType::Texture2D, 32, 16, 0, 0);
    bitmap.format = PixelFormat::Bc1Unorm;

    advance_mip(&mut sub, bitmap, 1);
    assert_eq!((2, 3, 12, 7), (sub.box_.left, sub.box_.top, sub.box_.right, sub.box_.bottom));

    advance_mip(&mut sub, bitmap, 2);
    assert_eq!((1, 3, 6, 7), (sub.box_.left, sub.box_.top, sub.box_.right, sub.box_.bottom));
}
