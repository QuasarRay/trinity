use bevy::app::{PluginGroup, PluginGroupBuilder};
use bevy::prelude::*;
use blue::BluePlugin;
use blueexposure::BlueExposurePlugin;
use trinity_meta::TrinityAppExt;
use trinityal::TrinityAlPlugin;

use crate::{
    EveChildContainer, EveChildMesh, EveChildPartData, EveSOF, EveSOFData, EveSOFDataFaction,
    EveSOFDataGeneric, EveSOFDataGenericShader, EveSOFDataHull, EveSOFDataHullArea,
    EveSOFDataHullLocatorSet, EveSOFDataHullSoundEmitter, EveSOFDataRace, EveSOFDataTransform,
    EveShip2, EveSpaceObject2, EveSpaceObjectChild, Tr2PPExecutionSlot, Tr2PPGenericEffect,
    Tr2PPParameter, TrinitySet,
};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Reflect, SystemSet)]
pub enum TrinitySet {
    CarbonHierarchy,
    ModularAssembly,
    PostProcess,
    RenderAbstraction,
}

#[derive(Default)]
pub struct TrinityPlugin;

macro_rules! register_carbon_classes {
    ($app:expr; $($ty:ty),+ $(,)?) => {{
        $(
            $app.register_trinity_class::<$ty>();
        )+
    }};
}

impl Plugin for TrinityPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EveSOF>()
            .init_resource::<EveSOFData>()
            .register_type::<TrinitySet>()
            .register_type::<Tr2PPExecutionSlot>()
            .register_type::<Tr2PPParameter>();

        register_carbon_classes!(
            app;
            EveSpaceObjectChild,
            EveChildMesh,
            EveChildContainer,
            EveShip2,
            EveSOFDataTransform,
            EveSOFDataHullArea,
            EveSOFDataHullLocatorSet,
            EveSOFDataHullSoundEmitter,
            EveSOFDataHull,
            EveSOFDataGenericShader,
            EveSOFDataGeneric,
            EveSOFDataFaction,
            EveSOFDataRace,
            EveSOFData,
            EveSOF,
            EveChildPartData,
            EveSpaceObject2,
            Tr2PPGenericEffect
        );

        app.configure_sets(
            Update,
            (
                TrinitySet::CarbonHierarchy,
                TrinitySet::ModularAssembly,
                TrinitySet::PostProcess,
                TrinitySet::RenderAbstraction,
            )
                .chain(),
        );
    }
}

pub struct TrinityPlugins;

impl PluginGroup for TrinityPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add_group(DefaultPlugins)
            .add(BlueExposurePlugin)
            .add(BluePlugin)
            .add(TrinityAlPlugin)
            .add(TrinityPlugin)
    }
}

pub struct TrinityVerificationPlugins;

impl PluginGroup for TrinityVerificationPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add_group(MinimalPlugins)
            .add(bevy::asset::AssetPlugin::default())
            .add(BlueExposurePlugin)
            .add(BluePlugin)
            .add(TrinityAlPlugin)
            .add(TrinityPlugin)
    }
}
