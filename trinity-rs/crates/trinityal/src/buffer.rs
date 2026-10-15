use bevy::{asset::Asset, prelude::*};
use serde::{Deserialize, Serialize};
use trinity_macros::TrinityClass;

use crate::{ALResult, E_INVALIDARG, E_INVALIDCALL, PixelFormat, S_OK, Tr2CpuUsage, Tr2GpuUsage};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Reflect, Serialize, Deserialize)]
pub struct Tr2BufferDescriptionAL {
    pub format: PixelFormat,
    pub stride: u32,
    pub count: u32,
    pub gpu_usage: Tr2GpuUsage,
    pub cpu_usage: Tr2CpuUsage,
}

impl Tr2BufferDescriptionAL {
    #[must_use]
    pub const fn from_stride(
        stride: u32,
        count: u32,
        gpu_usage: Tr2GpuUsage,
        cpu_usage: Tr2CpuUsage,
    ) -> Self {
        Self {
            format: PixelFormat::Unknown,
            stride,
            count,
            gpu_usage,
            cpu_usage,
        }
    }

    #[must_use]
    pub const fn size(self) -> u32 {
        self.stride.saturating_mul(self.count)
    }
}

#[derive(Asset, Reflect, Clone, Debug)]
pub struct Tr2BufferAsset {
    pub desc: Tr2BufferDescriptionAL,
    pub bytes: Vec<u8>,
    pub name: String,
}

#[derive(Component, Reflect, Clone, Debug, Default, TrinityClass)]
#[reflect(Component)]
#[trinity(
    module = "trinity",
    name = "Tr2BufferAL",
    legacy_header = "trinityal/include/Tr2BufferAL.h",
    kind = "component"
)]
pub struct Tr2BufferAL {
    #[trinity(skip)]
    handle: Option<Handle<Tr2BufferAsset>>,
}

impl Tr2BufferAL {
    #[must_use]
    pub fn is_valid(&self) -> bool {
        self.handle.is_some()
    }

    pub fn create(
        &mut self,
        assets: &mut Assets<Tr2BufferAsset>,
        desc: Tr2BufferDescriptionAL,
        initial_data: Option<&[u8]>,
    ) -> ALResult {
        if desc.stride == 0 || desc.count == 0 {
            return ALResult::new(E_INVALIDARG);
        }
        let expected = desc.size() as usize;
        if desc.cpu_usage == Tr2CpuUsage::None && initial_data.is_none() {
            return ALResult::new(E_INVALIDCALL);
        }
        if initial_data.is_some_and(|data| data.len() < expected) {
            return ALResult::new(E_INVALIDARG);
        }
        let mut bytes = vec![0_u8; expected];
        if let Some(data) = initial_data {
            bytes.copy_from_slice(&data[..expected]);
        }
        self.handle = Some(assets.add(Tr2BufferAsset {
            desc,
            bytes,
            name: String::new(),
        }));
        ALResult::new(S_OK)
    }

    #[must_use]
    pub fn get_desc<'a>(
        &self,
        assets: &'a Assets<Tr2BufferAsset>,
    ) -> Option<&'a Tr2BufferDescriptionAL> {
        self.handle
            .as_ref()
            .and_then(|handle| assets.get(handle))
            .map(|asset| &asset.desc)
    }

    #[must_use]
    pub fn get_size(&self, assets: &Assets<Tr2BufferAsset>) -> u32 {
        self.get_desc(assets).map_or(0, |desc| desc.size())
    }

    pub fn read_bytes<'a>(&self, assets: &'a Assets<Tr2BufferAsset>) -> Result<&'a [u8], ALResult> {
        let asset = self
            .handle
            .as_ref()
            .and_then(|handle| assets.get(handle))
            .ok_or(ALResult::new(E_INVALIDCALL))?;
        if !asset.desc.cpu_usage.can_read() {
            return Err(ALResult::new(E_INVALIDCALL));
        }
        Ok(&asset.bytes)
    }

    pub fn update_buffer(
        &mut self,
        assets: &mut Assets<Tr2BufferAsset>,
        offset: u32,
        data: &[u8],
    ) -> ALResult {
        let Some(asset) = self
            .handle
            .as_ref()
            .and_then(|handle| assets.get_mut(handle))
        else {
            return ALResult::new(E_INVALIDCALL);
        };
        if !asset.desc.cpu_usage.can_write() {
            return ALResult::new(E_INVALIDCALL);
        }
        let start = offset as usize;
        let Some(end) = start.checked_add(data.len()) else {
            return ALResult::new(E_INVALIDARG);
        };
        if end > asset.bytes.len() {
            return ALResult::new(E_INVALIDARG);
        }
        asset.bytes[start..end].copy_from_slice(data);
        ALResult::new(S_OK)
    }

    pub fn set_name(
        &mut self,
        assets: &mut Assets<Tr2BufferAsset>,
        name: impl Into<String>,
    ) -> ALResult {
        let Some(asset) = self
            .handle
            .as_ref()
            .and_then(|handle| assets.get_mut(handle))
        else {
            return ALResult::new(E_INVALIDCALL);
        };
        asset.name = name.into();
        ALResult::new(S_OK)
    }
}
