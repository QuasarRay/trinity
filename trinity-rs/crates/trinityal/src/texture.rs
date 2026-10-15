use bevy::{asset::Asset, prelude::*};
use serde::{Deserialize, Serialize};
use trinity_macros::TrinityClass;

use crate::{ALResult, Tr2BitmapDimensions, Tr2CpuUsage, Tr2GpuUsage, Tr2MsaaDesc, E_INVALIDARG, E_INVALIDCALL, S_OK};

#[derive(Asset, Reflect, Clone, Debug)]
pub struct Tr2TextureAsset {
    pub desc: Tr2BitmapDimensions,
    pub msaa: Tr2MsaaDesc,
    pub gpu_usage: Tr2GpuUsage,
    pub cpu_usage: Tr2CpuUsage,
    pub bytes: Vec<u8>,
    pub name: String,
}

#[derive(Component, Reflect, Clone, Debug, Default, TrinityClass)]
#[reflect(Component)]
#[trinity(module = "trinity", name = "Tr2TextureAL", legacy_header = "trinityal/include/Tr2TextureAL.h", kind = "component")]
pub struct Tr2TextureAL {
    #[trinity(skip)]
    handle: Option<Handle<Tr2TextureAsset>>,
}

impl Tr2TextureAL {
    #[must_use]
    pub fn is_valid(&self) -> bool { self.handle.is_some() }

    pub fn create(
        &mut self,
        assets: &mut Assets<Tr2TextureAsset>,
        desc: Tr2BitmapDimensions,
        msaa: Tr2MsaaDesc,
        gpu_usage: Tr2GpuUsage,
        cpu_usage: Tr2CpuUsage,
        initial_data: Option<&[u8]>,
    ) -> ALResult {
        if desc.width == 0 || desc.height == 0 || desc.get_true_mip_count() == 0 || desc.format.bits_per_pixel() == 0 {
            return ALResult::new(E_INVALIDARG);
        }
        let expected = desc.get_mip_size(0) as usize;
        if initial_data.is_some_and(|data| data.len() < expected) {
            return ALResult::new(E_INVALIDARG);
        }
        let mut bytes = vec![0_u8; expected];
        if let Some(data) = initial_data {
            bytes.copy_from_slice(&data[..expected]);
        }
        self.handle = Some(assets.add(Tr2TextureAsset { desc, msaa, gpu_usage, cpu_usage, bytes, name: String::new() }));
        ALResult::new(S_OK)
    }

    #[must_use]
    pub fn get_desc<'a>(&self, assets: &'a Assets<Tr2TextureAsset>) -> Option<&'a Tr2BitmapDimensions> {
        self.handle.as_ref().and_then(|handle| assets.get(handle)).map(|asset| &asset.desc)
    }

    #[must_use]
    pub fn get_width(&self, assets: &Assets<Tr2TextureAsset>) -> u32 { self.get_desc(assets).map_or(0, |desc| desc.width) }
    #[must_use]
    pub fn get_height(&self, assets: &Assets<Tr2TextureAsset>) -> u32 { self.get_desc(assets).map_or(0, |desc| desc.height) }
    #[must_use]
    pub fn get_depth(&self, assets: &Assets<Tr2TextureAsset>) -> u32 { self.get_desc(assets).map_or(0, |desc| desc.volume_depth) }
    #[must_use]
    pub fn get_mip_count(&self, assets: &Assets<Tr2TextureAsset>) -> u32 { self.get_desc(assets).map_or(0, |desc| desc.mip_count) }
    #[must_use]
    pub fn get_true_mip_count(&self, assets: &Assets<Tr2TextureAsset>) -> u32 { self.get_desc(assets).map_or(0, |desc| desc.get_true_mip_count()) }

    pub fn read_bytes<'a>(&self, assets: &'a Assets<Tr2TextureAsset>) -> Result<&'a [u8], ALResult> {
        let asset = self.handle.as_ref().and_then(|handle| assets.get(handle)).ok_or(ALResult::new(E_INVALIDCALL))?;
        if !asset.cpu_usage.can_read() { return Err(ALResult::new(E_INVALIDCALL)); }
        Ok(&asset.bytes)
    }

    pub fn set_name(&mut self, assets: &mut Assets<Tr2TextureAsset>, name: impl Into<String>) -> ALResult {
        let Some(asset) = self.handle.as_ref().and_then(|handle| assets.get_mut(handle)) else { return ALResult::new(E_INVALIDCALL); };
        asset.name = name.into();
        ALResult::new(S_OK)
    }
}
