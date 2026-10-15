use trinity_macros::trinity_test;
use trinityal::{PixelFormat, TextureType, Tr2BitmapDimensions};

fn dimensions(texture_type: TextureType, width: u32, height: u32, depth: u32, mip_count: u32) -> Tr2BitmapDimensions {
    Tr2BitmapDimensions { texture_type, width, height, volume_depth: depth, mip_count, ..Tr2BitmapDimensions::default() }
}

#[trinity_test(legacy = "trinityal/tests/BitmapDimensions.cpp::BitmapDimensions::InitialSizeIsZero", verification = "kani")]
fn initial_size_is_zero() {
    let dim = Tr2BitmapDimensions::default();
    assert_eq!(0, dim.get_width());
    assert_eq!(0, dim.get_height());
    assert_eq!(0, dim.get_depth());
    assert_eq!(0, dim.get_true_mip_count());
    assert_eq!(PixelFormat::Unknown, dim.get_format());
    assert_eq!(TextureType::Invalid, dim.get_type());
}

#[trinity_test(legacy = "trinityal/tests/BitmapDimensions.cpp::BitmapDimensions::ReportsCorrect2DSizes", verification = "kani")]
fn reports_correct_2d_sizes() {
    let dim = dimensions(TextureType::Texture2D, 32, 16, 0, 2);
    assert_eq!(32, dim.get_width());
    assert_eq!(16, dim.get_height());
    assert_eq!(0, dim.get_depth());
    assert_eq!(2, dim.get_true_mip_count());
}

#[trinity_test(legacy = "trinityal/tests/BitmapDimensions.cpp::BitmapDimensions::ReportsCorrect3DSizes", verification = "kani")]
fn reports_correct_3d_sizes() {
    let dim = dimensions(TextureType::Texture3D, 32, 16, 64, 2);
    assert_eq!(64, dim.get_depth());
    assert_eq!(2, dim.get_true_mip_count());
}

#[trinity_test(legacy = "trinityal/tests/BitmapDimensions.cpp::BitmapDimensions::CalculatesTrueMipCountAndMipSizes", verification = "kani")]
fn calculates_true_mip_count_and_mip_sizes() {
    let dim = dimensions(TextureType::Texture3D, 32, 16, 64, 0);
    assert_eq!(6, dim.get_true_mip_count());
    assert_eq!(32, dim.get_mip_width(0));
    assert_eq!(16, dim.get_mip_height(0));
    assert_eq!(64, dim.get_mip_depth(0));
    assert_eq!(1, dim.get_mip_width(5));
    assert_eq!(1, dim.get_mip_height(5));
    assert_eq!(2, dim.get_mip_depth(5));
}

#[trinity_test(legacy = "trinityal/tests/BitmapDimensions.cpp::BitmapDimensions::GettingMipSizesForIncorrectMipReturnsZero", verification = "kani")]
fn getting_mip_sizes_for_incorrect_mip_returns_zero() {
    let dim = dimensions(TextureType::Texture3D, 32, 16, 64, 0);
    assert_eq!(0, dim.get_mip_width(10));
    assert_eq!(0, dim.get_mip_height(10));
    assert_eq!(0, dim.get_mip_depth(10));
}

#[trinity_test(legacy = "trinityal/tests/BitmapDimensions.cpp::BitmapDimensions::CalculatesTrueMipCountAndMipSizesForNptTexture", verification = "kani")]
fn calculates_true_mip_count_for_npt_texture() {
    let dim = dimensions(TextureType::Texture2D, 731, 317, 0, 0);
    assert_eq!(10, dim.get_true_mip_count());
    assert_eq!(731, dim.get_mip_width(0));
    assert_eq!(365, dim.get_mip_width(1));
    assert_eq!(1, dim.get_mip_width(9));
}

#[trinity_test(legacy = "trinityal/tests/BitmapDimensions.cpp::BitmapDimensions::CalculatesCorrectMipSize", verification = "kani")]
fn calculates_correct_mip_size() {
    let mut dim = dimensions(TextureType::Texture2D, 32, 16, 0, 0);
    dim.format = PixelFormat::B8G8R8A8Unorm;
    assert_eq!(2048, dim.get_mip_size(0));
    assert_eq!(512, dim.get_mip_size(1));
    dim.format = PixelFormat::R16G16B16A16Float;
    assert_eq!(4096, dim.get_mip_size(0));
    dim.format = PixelFormat::Bc1Unorm;
    assert_eq!(256, dim.get_mip_size(0));
}
