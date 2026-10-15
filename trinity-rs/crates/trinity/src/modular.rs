use bevy::prelude::*;
use trinity_macros::TrinityClass;

#[derive(Clone, Copy, Debug, Reflect, PartialEq)]
pub struct EveSOFDataTransform {
    pub position: Vec3,
    pub rotation: Quat,
    pub scale: Vec3,
}

impl Default for EveSOFDataTransform {
    fn default() -> Self {
        Self {
            position: Vec3::ZERO,
            rotation: Quat::IDENTITY,
            scale: Vec3::ONE,
        }
    }
}

#[derive(Clone, Debug, Default, Reflect, PartialEq)]
pub struct EveSOFDataHullArea {
    pub name: String,
    pub shader: String,
    pub index: u32,
    pub count: u32,
}

#[derive(Clone, Debug, Default, Reflect, PartialEq)]
pub struct EveSOFDataHullLocatorSet {
    pub name: String,
    pub locators: Vec<EveSOFDataTransform>,
}

#[derive(Clone, Debug, Default, Reflect, PartialEq)]
pub struct EveSOFDataHullSoundEmitter {
    pub name: String,
    pub position: Vec3,
}

#[derive(Clone, Debug, Default, Reflect, PartialEq)]
pub struct EveSOFDataHull {
    pub name: String,
    pub geometry_res_file_path: String,
    pub bounding_sphere: Vec4,
    pub is_skinned: bool,
    pub opaque_areas: Vec<EveSOFDataHullArea>,
    pub locator_sets: Vec<EveSOFDataHullLocatorSet>,
    pub sound_emitters: Vec<EveSOFDataHullSoundEmitter>,
}

#[derive(Clone, Debug, Default, Reflect, PartialEq)]
pub struct EveSOFDataGenericShader {
    pub shader: String,
}

#[derive(Clone, Debug, Default, Reflect, PartialEq)]
pub struct EveSOFDataGeneric {
    pub area_shaders: Vec<EveSOFDataGenericShader>,
}

#[derive(Clone, Debug, Default, Reflect, PartialEq)]
pub struct EveSOFDataFaction { pub name: String }
#[derive(Clone, Debug, Default, Reflect, PartialEq)]
pub struct EveSOFDataRace { pub name: String }

#[derive(Resource, Reflect, Clone, Debug, Default, TrinityClass)]
#[reflect(Resource)]
#[trinity(module = "trinity", name = "EveSOFData", legacy_header = "trinity/EveSOFData.h", kind = "resource")]
pub struct EveSOFData {
    #[trinity(read, write, persist)]
    pub generic: EveSOFDataGeneric,
    #[trinity(read, write, persist)]
    pub hull: Vec<EveSOFDataHull>,
    #[trinity(read, write, persist)]
    pub faction: Vec<EveSOFDataFaction>,
    #[trinity(read, write, persist)]
    pub race: Vec<EveSOFDataRace>,
}

impl EveSOFData {
    #[must_use]
    pub fn hull_named(&self, name: &str) -> Option<&EveSOFDataHull> {
        self.hull.iter().find(|hull| hull.name == name)
    }
}

#[derive(Clone, Debug, Reflect, PartialEq)]
pub struct EveChildPartData {
    pub part_tag: u32,
    pub hull_name: String,
    pub transform: EveSOFDataTransform,
    pub bounding_radius: f32,
}

#[derive(Component, Reflect, Clone, Debug, Default, TrinityClass)]
#[reflect(Component)]
#[trinity(module = "trinity", name = "EveSpaceObject2", legacy_header = "trinity/EveSpaceObject2.h", kind = "component")]
pub struct EveSpaceObject2 {
    #[trinity(read, persist)]
    pub parts: Vec<EveChildPartData>,
    #[trinity(read, persist)]
    pub bounding_radius: f32,
    #[trinity(skip)]
    next_part_tag: u32,
}

impl EveSpaceObject2 {
    #[must_use]
    pub fn add_hull(&mut self, hull: &EveSOFDataHull) -> u32 {
        self.next_part_tag = self.next_part_tag.max(1);
        let part_tag = self.next_part_tag;
        self.next_part_tag = self.next_part_tag.saturating_add(1);
        self.parts.push(EveChildPartData {
            part_tag,
            hull_name: hull.name.clone(),
            transform: EveSOFDataTransform::default(),
            bounding_radius: hull.bounding_sphere.w,
        });
        self.recompute_bounds();
        part_tag
    }

    pub fn remove_part(&mut self, part_tag: u32) -> bool {
        let before = self.parts.len();
        self.parts.retain(|part| part.part_tag != part_tag);
        self.recompute_bounds();
        before != self.parts.len()
    }

    pub fn set_transform(&mut self, part_tag: u32, transform: EveSOFDataTransform) -> bool {
        let Some(part) = self.parts.iter_mut().find(|part| part.part_tag == part_tag) else {
            return false;
        };
        part.transform = transform;
        true
    }

    pub fn recompute_bounds(&mut self) {
        self.bounding_radius = self
            .parts
            .iter()
            .map(|part| part.bounding_radius * part.transform.scale.max_element().abs())
            .fold(0.0_f32, f32::max);
    }
}

#[derive(Resource, Reflect, Clone, Debug, Default, TrinityClass)]
#[reflect(Resource)]
#[trinity(module = "trinity", name = "EveSOF", legacy_header = "trinity/EveSOF.h", kind = "resource")]
pub struct EveSOF {
    #[trinity(read, write, persist)]
    pub data: EveSOFData,
}

impl EveSOF {
    pub fn set_data(&mut self, data: EveSOFData) {
        self.data = data;
    }

    #[must_use]
    pub fn create_modular_object(&self, world: &mut World, hull_name: &str) -> Option<Entity> {
        let hull = self.data.hull_named(hull_name)?;
        let mut object = EveSpaceObject2::default();
        object.add_hull(hull);
        Some(world.spawn(object).id())
    }

    pub fn add_hull(&self, world: &mut World, entity: Entity, hull_name: &str) -> Option<u32> {
        let hull = self.data.hull_named(hull_name)?;
        world.get_mut::<EveSpaceObject2>(entity).map(|mut object| object.add_hull(hull))
    }
}
