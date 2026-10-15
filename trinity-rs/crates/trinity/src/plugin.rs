use bevy::app::{PluginGroup, PluginGroupBuilder};
use bevy::prelude::*;
use blue::BluePlugin;
use blueexposure::BlueExposurePlugin;
use trinity_meta::TrinityAppExt;
use trinityal::TrinityAlPlugin;
use crate::{EveChildContainer, EveChildMesh, EveSOF, EveSOFData, EveShip2, EveSpaceObject2, EveSpaceObjectChild, Tr2PPExecutionSlot, Tr2PPGenericEffect, Tr2PPParameter};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Reflect, SystemSet)]
pub enum TrinitySet { CarbonHierarchy, ModularAssembly, PostProcess, RenderAbstraction }
#[derive(Default)] pub struct TrinityPlugin;
impl Plugin for TrinityPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EveSOF>().init_resource::<EveSOFData>()
            .register_type::<TrinitySet>().register_type::<Tr2PPExecutionSlot>().register_type::<Tr2PPParameter>()
            .register_trinity_class::<EveSpaceObjectChild>().register_trinity_class::<EveChildMesh>()
            .register_trinity_class::<EveChildContainer>().register_trinity_class::<EveShip2>()
            .register_trinity_class::<EveSOFData>().register_trinity_class::<EveSOF>()
            .register_trinity_class::<EveSpaceObject2>().register_trinity_class::<Tr2PPGenericEffect>()
            .configure_sets(Update, (TrinitySet::CarbonHierarchy, TrinitySet::ModularAssembly, TrinitySet::PostProcess, TrinitySet::RenderAbstraction).chain());
    }
}
pub struct TrinityPlugins;
impl PluginGroup for TrinityPlugins {
    fn build(self) -> PluginGroupBuilder { PluginGroupBuilder::start::<Self>().add_group(DefaultPlugins).add(BlueExposurePlugin).add(BluePlugin).add(TrinityAlPlugin).add(TrinityPlugin) }
}
pub struct TrinityVerificationPlugins;
impl PluginGroup for TrinityVerificationPlugins {
    fn build(self) -> PluginGroupBuilder { PluginGroupBuilder::start::<Self>().add_group(MinimalPlugins).add(bevy::asset::AssetPlugin::default()).add(BlueExposurePlugin).add(BluePlugin).add(TrinityAlPlugin).add(TrinityPlugin) }
}
