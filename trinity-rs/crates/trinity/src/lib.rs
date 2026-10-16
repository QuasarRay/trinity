extern crate self as trinity_rs;

mod hierarchy;
mod modular;
mod plugin;
mod postprocess;
mod runtime;

pub use hierarchy::{
    EveChildContainer, EveChildMesh, EveChildRelations, EveShip2, EveSpaceObjectChild,
    TrinityHierarchyWorldExt,
};
pub use modular::{
    EveChildPartData, EveModularObjectModifier, EveModularPartData, EveSOF, EveSOFData,
    EveSOFDataFaction, EveSOFDataGeneric, EveSOFDataGenericShader, EveSOFDataHull,
    EveSOFDataHullArea, EveSOFDataHullLocatorSet, EveSOFDataHullSoundEmitter, EveSOFDataRace,
    EveSOFDataTransform, EveSpaceObject2, GetInvalidPartTag, INVALID_PART_TAG, ModularError,
};
pub use plugin::{TrinityPlugin, TrinityPlugins, TrinitySet, TrinityVerificationPlugins};
pub use postprocess::{Tr2PPExecutionSlot, Tr2PPGenericEffect, Tr2PPParameter};
pub use runtime::TrinityRuntime;
pub use trinityal;

pub mod prelude {
    pub use blue::prelude::*;
    pub use trinity_meta::prelude::*;
    pub use trinityal::prelude::*;

    pub use crate::{
        EveChildContainer, EveChildMesh, EveChildPartData, EveChildRelations,
        EveModularObjectModifier, EveModularPartData, EveSOF, EveSOFData, EveSOFDataFaction,
        EveSOFDataGeneric, EveSOFDataGenericShader, EveSOFDataHull, EveSOFDataHullArea,
        EveSOFDataHullLocatorSet, EveSOFDataHullSoundEmitter, EveSOFDataRace, EveSOFDataTransform,
        EveShip2, EveSpaceObject2, EveSpaceObjectChild, GetInvalidPartTag, INVALID_PART_TAG,
        ModularError, Tr2PPExecutionSlot, Tr2PPGenericEffect, Tr2PPParameter,
        TrinityHierarchyWorldExt, TrinityPlugin, TrinityPlugins, TrinityRuntime, TrinitySet,
        TrinityVerificationPlugins,
    };
}
