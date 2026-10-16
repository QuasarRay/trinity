use bevy::prelude::Reflect;
use serde::{Deserialize, Serialize};

use crate::Tr2BitmapDimensions;

const FULL_RANGE: u32 = u32::MAX;

#[derive(Clone, Copy, Debug, Default, PartialEq, Reflect, Serialize, Deserialize)]
pub struct Tr2Viewport {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub min_z: f32,
    pub max_z: f32,
}

impl Tr2Viewport {
    #[must_use]
    #[allow(clippy::cast_precision_loss)]
    pub const fn from_size(width: u32, height: u32) -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            width: width as f32,
            height: height as f32,
            min_z: 0.0,
            max_z: 1.0,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Reflect, Serialize, Deserialize)]
pub struct Tr2TextureCoordBox {
    pub left: u32,
    pub top: u32,
    pub front: u32,
    pub right: u32,
    pub bottom: u32,
    pub back: u32,
}

impl Tr2TextureCoordBox {
    #[must_use]
    pub const fn get_width(self) -> u32 {
        self.right.wrapping_sub(self.left)
    }

    #[must_use]
    pub const fn get_height(self) -> u32 {
        self.bottom.wrapping_sub(self.top)
    }

    #[must_use]
    pub const fn get_depth(self) -> u32 {
        self.back.wrapping_sub(self.front)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Reflect, Serialize, Deserialize)]
pub struct Tr2MsaaDesc {
    pub samples: u32,
    pub quality: u32,
}

impl Default for Tr2MsaaDesc {
    fn default() -> Self {
        Self {
            samples: 1,
            quality: 0,
        }
    }
}

impl Tr2MsaaDesc {
    #[must_use]
    pub const fn new(samples: u32, quality: u32) -> Self {
        Self {
            samples: if samples == 0 { 1 } else { samples },
            quality,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Reflect, Serialize, Deserialize)]
pub struct Tr2TextureSubresource {
    pub start_face: u32,
    pub end_face: u32,
    pub start_mip_level: u32,
    pub end_mip_level: u32,
    pub box_: Tr2TextureCoordBox,
}

impl Default for Tr2TextureSubresource {
    fn default() -> Self {
        Self {
            start_face: 0,
            end_face: FULL_RANGE,
            start_mip_level: 0,
            end_mip_level: FULL_RANGE,
            box_: sentinel_box(),
        }
    }
}

impl Tr2TextureSubresource {
    #[must_use]
    pub const fn mip(mip_level: u32) -> Self {
        Self {
            start_face: 0,
            end_face: 1,
            start_mip_level: mip_level,
            end_mip_level: mip_level + 1,
            box_: sentinel_box(),
        }
    }

    #[must_use]
    pub const fn face_mip(face: u32, mip_level: u32) -> Self {
        Self {
            start_face: face,
            end_face: face + 1,
            start_mip_level: mip_level,
            end_mip_level: mip_level + 1,
            box_: sentinel_box(),
        }
    }

    #[must_use]
    pub const fn get_width(self) -> u32 {
        self.box_.get_width()
    }

    #[must_use]
    pub const fn get_height(self) -> u32 {
        self.box_.get_height()
    }

    #[must_use]
    pub const fn get_depth(self) -> u32 {
        self.box_.get_depth()
    }

    #[must_use]
    pub const fn get_mip_count(self) -> u32 {
        self.end_mip_level.wrapping_sub(self.start_mip_level)
    }

    #[must_use]
    pub const fn get_face_count(self) -> u32 {
        self.end_face.wrapping_sub(self.start_face)
    }

    #[must_use]
    pub const fn has_box(self) -> bool {
        self.box_.left != FULL_RANGE
            || self.box_.top != FULL_RANGE
            || self.box_.front != FULL_RANGE
            || self.box_.right != FULL_RANGE
            || self.box_.bottom != FULL_RANGE
            || self.box_.back != FULL_RANGE
    }

    #[must_use]
    pub const fn is_single_subresource(self) -> bool {
        self.end_face == self.start_face + 1
            && self.end_mip_level == self.start_mip_level + 1
    }

    pub fn set_box(&mut self, values: [u32; 6]) -> &mut Self {
        self.box_ = Tr2TextureCoordBox {
            left: values[0],
            top: values[1],
            front: values[2],
            right: values[3],
            bottom: values[4],
            back: values[5],
        };
        self
    }

    pub fn set_rect(&mut self, left: u32, top: u32, right: u32, bottom: u32) -> &mut Self {
        self.box_ = Tr2TextureCoordBox {
            left,
            top,
            front: 0,
            right,
            bottom,
            back: 1,
        };
        self
    }

    pub fn clamp_to_texture(&mut self, texture: Tr2BitmapDimensions) {
        let array_size = texture.get_array_size().max(1);
        self.start_face = self.start_face.min(array_size - 1);
        self.end_face = self.end_face.min(array_size);

        let mip_count = texture.get_true_mip_count();
        if mip_count == 0 {
            self.start_mip_level = 0;
            self.end_mip_level = 0;
            return;
        }

        self.start_mip_level = self.start_mip_level.min(mip_count - 1);
        self.end_mip_level = self.end_mip_level.min(mip_count);

        let mip_width = texture.get_mip_width(self.start_mip_level);
        let mip_height = texture.get_mip_height(self.start_mip_level);
        let mip_depth = texture
            .get_depth()
            .checked_shr(self.start_mip_level)
            .unwrap_or(0)
            .max(1);

        if self.has_box() {
            self.box_.left = self.box_.left.min(mip_width.saturating_sub(1));
            self.box_.right = self.box_.right.min(mip_width);
            self.box_.top = self.box_.top.min(mip_height.saturating_sub(1));
            self.box_.bottom = self.box_.bottom.min(mip_height);
            if matches!(texture.get_type(), crate::TextureType::Texture3D) {
                self.box_.front = self.box_.front.min(mip_depth.saturating_sub(1));
                self.box_.back = self.box_.back.min(mip_depth);
            } else {
                self.box_.front = 0;
                self.box_.back = 1;
            }
        } else {
            self.box_ = Tr2TextureCoordBox {
                left: 0,
                top: 0,
                front: 0,
                right: mip_width,
                bottom: mip_height,
                back: mip_depth,
            };
        }
    }

    #[must_use]
    pub fn is_subresource_full(self, texture: Tr2BitmapDimensions) -> bool {
        if self.start_face > 0 || self.end_face < texture.get_array_size() {
            return false;
        }
        if self.start_mip_level > 0 {
            return false;
        }
        if self.end_mip_level < texture.get_true_mip_count() {
            return false;
        }
        if self.has_box() {
            if self.box_.left > 0 || self.box_.top > 0 || self.box_.front > 0 {
                return false;
            }
            if self.box_.right < texture.get_width()
                || self.box_.bottom < texture.get_height()
                || self.box_.back < texture.get_depth()
            {
                return false;
            }
        }
        true
    }

    #[must_use]
    pub fn is_valid_for_bitmap(self, bitmap: Tr2BitmapDimensions) -> bool {
        if self.end_face > bitmap.get_array_size() {
            return false;
        }
        if self.end_mip_level > bitmap.get_true_mip_count() {
            return false;
        }
        if self.has_box() {
            if self.box_.right > bitmap.get_mip_width(self.start_mip_level) {
                return false;
            }
            if self.box_.bottom > bitmap.get_mip_height(self.start_mip_level) {
                return false;
            }
            if matches!(bitmap.get_type(), crate::TextureType::Texture3D)
                && self.box_.back > bitmap.get_mip_depth(self.start_mip_level)
            {
                return false;
            }
        }
        true
    }

    #[must_use]
    pub const fn is_valid(self) -> bool {
        if self.has_box()
            && (self.box_.left >= self.box_.right
                || self.box_.top >= self.box_.bottom
                || self.box_.front >= self.box_.back)
        {
            return false;
        }
        self.start_face < self.end_face && self.start_mip_level < self.end_mip_level
    }
}

#[must_use]
pub fn crop(
    source: &mut Tr2TextureSubresource,
    source_bitmap: Tr2BitmapDimensions,
    destination: &mut Tr2TextureSubresource,
    destination_bitmap: Tr2BitmapDimensions,
) -> bool {
    if destination.get_face_count() != source.get_face_count() {
        return false;
    }

    if source.has_box()
        && destination.has_box()
        && destination.get_depth() != source.get_depth()
    {
        return false;
    }

    source.clamp_to_texture(source_bitmap);
    destination.clamp_to_texture(destination_bitmap);

    if source.get_width() < destination.get_width() {
        destination.box_.right = destination.box_.left + source.get_width();
    } else {
        source.box_.right = source.box_.left + destination.get_width();
    }

    if source.get_height() < destination.get_height() {
        destination.box_.bottom = destination.box_.top + source.get_height();
    } else {
        source.box_.bottom = source.box_.top + destination.get_height();
    }

    source.clamp_to_texture(source_bitmap);
    destination.clamp_to_texture(destination_bitmap);

    source.is_valid() && destination.is_valid()
}

pub fn advance_mip(
    subresource: &mut Tr2TextureSubresource,
    bitmap: Tr2BitmapDimensions,
    mip: u32,
) {
    let start = subresource.start_mip_level;

    if bitmap.get_mip_width(start + mip + 1) < bitmap.get_mip_width(start + mip) {
        subresource.box_.left /= 2;
        subresource.box_.right /= 2;

        if bitmap.is_compressed() && subresource.get_width() < 4 {
            subresource.box_.right = subresource.box_.left + 4;
        }
    }

    if bitmap.get_mip_height(start + mip + 1) < bitmap.get_mip_height(start + mip) {
        subresource.box_.top /= 2;
        subresource.box_.bottom /= 2;

        if bitmap.is_compressed() && subresource.get_height() < 4 {
            subresource.box_.bottom = subresource.box_.top + 4;
        }
    }

    if bitmap.get_mip_depth(start + mip + 1) < bitmap.get_mip_depth(start + mip) {
        subresource.box_.front /= 2;
        subresource.box_.back /= 2;

        if bitmap.is_compressed() && subresource.get_depth() < 4 {
            subresource.box_.bottom = subresource.box_.top + 4;
        }
    }
}

#[allow(non_snake_case)]
#[must_use]
pub fn Crop(
    source: &mut Tr2TextureSubresource,
    source_bitmap: Tr2BitmapDimensions,
    destination: &mut Tr2TextureSubresource,
    destination_bitmap: Tr2BitmapDimensions,
) -> bool {
    crop(source, source_bitmap, destination, destination_bitmap)
}

#[allow(non_snake_case)]
pub fn AdvanceMip(
    subresource: &mut Tr2TextureSubresource,
    bitmap: Tr2BitmapDimensions,
    mip: u32,
) {
    advance_mip(subresource, bitmap, mip);
}

const fn sentinel_box() -> Tr2TextureCoordBox {
    Tr2TextureCoordBox {
        left: FULL_RANGE,
        top: FULL_RANGE,
        front: FULL_RANGE,
        right: FULL_RANGE,
        bottom: FULL_RANGE,
        back: FULL_RANGE,
    }
}
