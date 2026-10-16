use trinity_macros::trinity_test;
use trinityal::{PixelFormat, TextureType, Tr2BitmapDimensions};

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
        ..Default::default()
    }
}

#[trinity_test(
    legacy = "trinityal/tests/BitmapDimensions.cpp::BitmapDimensions::InitialSizeIsZero",
    verification = "kani"
)]
fn initial_size_is_zero() {
    let dimensions = Tr2BitmapDimensions::default();
    assert_eq!(0, dimensions.get_width());
    assert_eq!(0, dimensions.get_true_mip_count());
    assert_eq!(PixelFormat::Unknown, dimensions.get_format());
    assert_eq!(TextureType::Invalid, dimensions.get_type());
}

#[trinity_test(
    legacy = "trinityal/tests/BitmapDimensions.cpp::BitmapDimensions::ReportsCorrect2DSizes",
    verification = "kani"
)]
fn reports_correct_2d_sizes() {
    let dimensions = dimensions(TextureType::Texture2D, 32, 16, 0, 2);
    assert_eq!(
        (32, 16, 0, 2),
        (
            dimensions.get_width(),
            dimensions.get_height(),
            dimensions.get_depth(),
            dimensions.get_true_mip_count()
        )
    );
}

#[trinity_test(
    legacy = "trinityal/tests/BitmapDimensions.cpp::BitmapDimensions::ReportsCorrect3DSizes",
    verification = "kani"
)]
fn reports_correct_3d_sizes() {
    let dimensions = dimensions(TextureType::Texture3D, 32, 16, 64, 2);
    assert_eq!(64, dimensions.get_depth());
    assert_eq!(2, dimensions.get_true_mip_count());
}

#[trinity_test(
    legacy = "trinityal/tests/BitmapDimensions.cpp::BitmapDimensions::CalculatesTrueMipCountAndMipSizes",
    verification = "kani"
)]
fn calculates_true_mip_count_and_mip_sizes() {
    let dimensions = dimensions(TextureType::Texture3D, 32, 16, 64, 0);
    assert_eq!(6, dimensions.get_true_mip_count());
    assert_eq!(
        (32, 16, 64),
        (
            dimensions.get_mip_width(0),
            dimensions.get_mip_height(0),
            dimensions.get_mip_depth(0)
        )
    );
}

#[trinity_test(
    legacy = "trinityal/tests/BitmapDimensions.cpp::BitmapDimensions::GettingMipSizesForIncorrectMipReturnsZero",
    verification = "kani"
)]
fn getting_mip_sizes_for_incorrect_mip_returns_zero() {
    let dimensions = dimensions(TextureType::Texture3D, 32, 16, 64, 0);
    assert_eq!(
        (0, 0, 0),
        (
            dimensions.get_mip_width(10),
            dimensions.get_mip_height(10),
            dimensions.get_mip_depth(10)
        )
    );
}

#[trinity_test(
    legacy = "trinityal/tests/BitmapDimensions.cpp::BitmapDimensions::CalculatesTrueMipCountAndMipSizesForNptTexture",
    verification = "kani"
)]
fn calculates_true_mip_count_for_npt_texture() {
    let dimensions = dimensions(TextureType::Texture2D, 731, 317, 0, 0);
    assert_eq!(10, dimensions.get_true_mip_count());
    assert_eq!(365, dimensions.get_mip_width(1));
}

#[trinity_test(
    legacy = "trinityal/tests/BitmapDimensions.cpp::BitmapDimensions::CalculatesCorrectMipSize",
    verification = "kani"
)]
fn calculates_correct_mip_size() {
    let mut dimensions = dimensions(TextureType::Texture2D, 32, 16, 0, 0);
    dimensions.format = PixelFormat::B8G8R8A8Unorm;
    assert_eq!(2048, dimensions.get_mip_size(0));
    dimensions.format = PixelFormat::Bc1Unorm;
    assert_eq!(256, dimensions.get_mip_size(0));
}
