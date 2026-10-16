use bevy::ecs::reflect::{ReflectComponent, ReflectResource};
use bevy::prelude::*;
use trinity_macros::TrinityClass;

pub const INVALID_PART_TAG: u32 = u32::MAX;

#[derive(Clone, Copy, Debug, Reflect, PartialEq, TrinityClass)]
#[trinity(
    module = "trinity",
    name = "EveSOFDataTransform",
    legacy_header = "trinity/Eve/SpaceObjectFactory/EveSOFData.h",
    kind = "value"
)]
pub struct EveSOFDataTransform {
    #[trinity(read, write, persist)]
    pub position: Vec3,
    #[trinity(read, write, persist)]
    pub rotation: Quat,
    #[trinity(read, write, persist)]
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

#[derive(Clone, Debug, Default, Reflect, PartialEq, TrinityClass)]
#[trinity(
    module = "trinity",
    name = "EveSOFDataHullArea",
    legacy_header = "trinity/Eve/SpaceObjectFactory/EveSOFData.h",
    kind = "value"
)]
pub struct EveSOFDataHullArea {
    #[trinity(read, write, persist)]
    pub name: String,
    #[trinity(read, write, persist)]
    pub shader: String,
    #[trinity(read, write, persist)]
    pub index: u32,
    #[trinity(read, write, persist)]
    pub count: u32,
}

#[derive(Clone, Debug, Default, Reflect, PartialEq, TrinityClass)]
#[trinity(
    module = "trinity",
    name = "EveSOFDataHullLocatorSet",
    legacy_header = "trinity/Eve/SpaceObjectFactory/EveSOFData.h",
    kind = "value"
)]
pub struct EveSOFDataHullLocatorSet {
    #[trinity(read, write, persist)]
    pub name: String,
    #[trinity(read, write, persist)]
    pub locators: Vec<EveSOFDataTransform>,
}

#[derive(Clone, Debug, Default, Reflect, PartialEq, TrinityClass)]
#[trinity(
    module = "trinity",
    name = "EveSOFDataHullSoundEmitter",
    legacy_header = "trinity/Eve/SpaceObjectFactory/EveSOFData.h",
    kind = "value"
)]
pub struct EveSOFDataHullSoundEmitter {
    #[trinity(read, write, persist)]
    pub name: String,
    #[trinity(read, write, persist)]
    pub position: Vec3,
}

#[derive(Clone, Debug, Default, Reflect, PartialEq, TrinityClass)]
#[trinity(
    module = "trinity",
    name = "EveSOFDataHull",
    legacy_header = "trinity/Eve/SpaceObjectFactory/EveSOFData.h",
    kind = "value"
)]
pub struct EveSOFDataHull {
    #[trinity(read, write, persist)]
    pub name: String,
    #[trinity(read, write, persist)]
    pub geometry_res_file_path: String,
    #[trinity(read, write, persist)]
    pub bounding_sphere: Vec4,
    #[trinity(read, write, persist)]
    pub is_skinned: bool,
    #[trinity(read, write, persist)]
    pub opaque_areas: Vec<EveSOFDataHullArea>,
    #[trinity(read, write, persist)]
    pub locator_sets: Vec<EveSOFDataHullLocatorSet>,
    #[trinity(read, write, persist)]
    pub sound_emitters: Vec<EveSOFDataHullSoundEmitter>,
}

#[derive(Clone, Debug, Default, Reflect, PartialEq, TrinityClass)]
#[trinity(
    module = "trinity",
    name = "EveSOFDataGenericShader",
    legacy_header = "trinity/Eve/SpaceObjectFactory/EveSOFData.h",
    kind = "value"
)]
pub struct EveSOFDataGenericShader {
    #[trinity(read, write, persist)]
    pub shader: String,
}

#[derive(Clone, Debug, Default, Reflect, PartialEq, TrinityClass)]
#[trinity(
    module = "trinity",
    name = "EveSOFDataGeneric",
    legacy_header = "trinity/Eve/SpaceObjectFactory/EveSOFData.h",
    kind = "value"
)]
pub struct EveSOFDataGeneric {
    #[trinity(read, write, persist)]
    pub area_shaders: Vec<EveSOFDataGenericShader>,
}

#[derive(Clone, Debug, Default, Reflect, PartialEq, TrinityClass)]
#[trinity(
    module = "trinity",
    name = "EveSOFDataFaction",
    legacy_header = "trinity/Eve/SpaceObjectFactory/EveSOFData.h",
    kind = "value"
)]
pub struct EveSOFDataFaction {
    #[trinity(read, write, persist)]
    pub name: String,
}

#[derive(Clone, Debug, Default, Reflect, PartialEq, TrinityClass)]
#[trinity(
    module = "trinity",
    name = "EveSOFDataRace",
    legacy_header = "trinity/Eve/SpaceObjectFactory/EveSOFData.h",
    kind = "value"
)]
pub struct EveSOFDataRace {
    #[trinity(read, write, persist)]
    pub name: String,
}

#[derive(Resource, Reflect, Clone, Debug, Default, TrinityClass)]
#[reflect(Resource)]
#[trinity(
    module = "trinity",
    name = "EveSOFData",
    legacy_header = "trinity/Eve/SpaceObjectFactory/EveSOFData.h",
    kind = "resource"
)]
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
pub struct EveModularPartData {
    pub part_tag: u32,
    pub hull_name: String,
    pub effective_faction: String,
    pub effective_race: String,
    pub transform: EveSOFDataTransform,
    pub local_bounding_sphere: Vec4,
    pub geometry_res_file_path: String,
    pub is_skinned: bool,
    pub damage_locators: Vec<Vec3>,
    pub has_sound_emitters: bool,
}

impl EveModularPartData {
    #[must_use]
    fn transformed_sphere(&self) -> ModularSphere {
        let local_center = self.local_bounding_sphere.truncate();
        let scaled_center = local_center * self.transform.scale;
        let center = self.transform.position + self.transform.rotation * scaled_center;
        let scale = self.transform.scale.abs().max_element();
        ModularSphere {
            center,
            radius: self.local_bounding_sphere.w.abs() * scale,
        }
    }

    #[must_use]
    fn transformed_damage_locators(&self) -> impl Iterator<Item = Vec3> + '_ {
        self.damage_locators.iter().copied().map(|position| {
            self.transform.position
                + self.transform.rotation * (position * self.transform.scale)
        })
    }
}

#[derive(Component, Reflect, Clone, Debug, Default, TrinityClass)]
#[reflect(Component)]
#[trinity(
    module = "trinity",
    name = "EveChildPartData",
    legacy_header = "trinity/Eve/SpaceObject/Children/EveChildPartData.h",
    kind = "component"
)]
pub struct EveChildPartData {
    #[trinity(read, write, persist)]
    pub faction: String,
    #[trinity(read, write, persist)]
    pub race: String,
    #[trinity(read, persist)]
    pub parts: Vec<EveModularPartData>,
}

impl EveChildPartData {
    #[must_use]
    pub fn get_unused_part_id(&self) -> u32 {
        self.parts
            .iter()
            .fold(1, |next, part| next.max(part.part_tag.saturating_add(1)))
    }

    #[allow(non_snake_case)]
    #[must_use]
    pub fn GetUnusedPartID(&self) -> u32 {
        self.get_unused_part_id()
    }
}

#[derive(Component, Reflect, Clone, Debug, Default, TrinityClass)]
#[reflect(Component)]
#[trinity(
    module = "trinity",
    name = "EveSpaceObject2",
    legacy_header = "trinity/Eve/SpaceObject/EveSpaceObject2.h",
    kind = "component"
)]
pub struct EveSpaceObject2 {
    #[trinity(read, persist)]
    pub part_data: EveChildPartData,
    #[trinity(read, persist)]
    pub bounding_sphere_center: Vec3,
    #[trinity(read, persist)]
    pub bounding_sphere_radius: f32,
    #[trinity(read, persist)]
    pub shape_ellipsoid_center: Vec3,
}

impl EveSpaceObject2 {
    #[must_use]
    pub fn parts(&self) -> &[EveModularPartData] {
        &self.part_data.parts
    }

    #[must_use]
    pub fn part(&self, part_tag: u32) -> Option<&EveModularPartData> {
        self.part_data
            .parts
            .iter()
            .find(|part| part.part_tag == part_tag)
    }

    fn part_mut(&mut self, part_tag: u32) -> Option<&mut EveModularPartData> {
        self.part_data
            .parts
            .iter_mut()
            .find(|part| part.part_tag == part_tag)
    }

    pub fn recompute_bounds(&mut self) {
        let mut spheres = self
            .part_data
            .parts
            .iter()
            .map(EveModularPartData::transformed_sphere)
            .collect::<Vec<_>>();
        spheres.sort_by(|left, right| right.radius.total_cmp(&left.radius));

        let mut combined: Option<ModularSphere> = None;
        let mut minimum = Vec3::splat(f32::INFINITY);
        let mut maximum = Vec3::splat(f32::NEG_INFINITY);

        for sphere in spheres {
            combined = Some(match combined {
                None => sphere,
                Some(current) => current.include(sphere),
            });
            let radius = Vec3::splat(sphere.radius);
            minimum = minimum.min(sphere.center - radius);
            maximum = maximum.max(sphere.center + radius);
        }

        if let Some(sphere) = combined {
            self.bounding_sphere_center = sphere.center;
            self.bounding_sphere_radius = sphere.radius;
            self.shape_ellipsoid_center = (minimum + maximum) * 0.5;
        } else {
            self.bounding_sphere_center = Vec3::ZERO;
            self.bounding_sphere_radius = 0.0;
            self.shape_ellipsoid_center = Vec3::ZERO;
        }
    }

    #[must_use]
    pub fn damage_locators(&self) -> Vec<Vec3> {
        self.part_data
            .parts
            .iter()
            .flat_map(EveModularPartData::transformed_damage_locators)
            .collect()
    }

    #[must_use]
    pub fn damage_locator_count(&self) -> usize {
        self.part_data
            .parts
            .iter()
            .map(|part| part.damage_locators.len())
            .sum()
    }

    #[must_use]
    pub fn damage_locator(&self, index: usize) -> Option<Vec3> {
        self.damage_locators().get(index).copied()
    }

    #[must_use]
    pub fn animated_hull_count(&self) -> usize {
        self.part_data
            .parts
            .iter()
            .filter(|part| part.is_skinned)
            .count()
    }

    #[must_use]
    pub fn audio_container_count(&self) -> usize {
        self.part_data
            .parts
            .iter()
            .filter(|part| part.has_sound_emitters)
            .count()
    }

    #[must_use]
    pub fn audio_container_part_tags(&self) -> Vec<u32> {
        self.part_data
            .parts
            .iter()
            .filter(|part| part.has_sound_emitters)
            .map(|part| part.part_tag)
            .collect()
    }

    #[must_use]
    pub fn audio_container_names(&self) -> Vec<&str> {
        self.part_data
            .parts
            .iter()
            .filter(|part| part.has_sound_emitters)
            .map(|part| part.hull_name.as_str())
            .collect()
    }

    #[must_use]
    pub fn static_mesh_groups(&self) -> Vec<(String, usize)> {
        let mut groups: Vec<(String, usize)> = Vec::new();
        for part in self.part_data.parts.iter().filter(|part| !part.is_skinned) {
            if let Some((_, count)) = groups
                .iter_mut()
                .find(|(path, _)| path == &part.geometry_res_file_path)
            {
                *count += 1;
            } else {
                groups.push((part.geometry_res_file_path.clone(), 1));
            }
        }
        groups
    }

    #[must_use]
    pub fn static_mesh_count(&self) -> usize {
        self.static_mesh_groups().len()
    }

    #[must_use]
    pub fn static_instance_count(&self, mesh_index: usize) -> Option<usize> {
        self.static_mesh_groups()
            .get(mesh_index)
            .map(|(_, count)| *count)
    }

    #[must_use]
    pub fn static_mesh_path(&self, mesh_index: usize) -> Option<String> {
        self.static_mesh_groups()
            .get(mesh_index)
            .map(|(path, _)| path.clone())
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Reflect)]
pub enum ModularError {
    KeyError,
}

pub struct EveModularObjectModifier<'w> {
    object: Mut<'w, EveSpaceObject2>,
    sof_data: EveSOFData,
}

impl EveModularObjectModifier<'_> {
    #[must_use]
    pub const fn invalid_part_tag() -> u32 {
        INVALID_PART_TAG
    }

    pub fn add_hull(
        &mut self,
        hull_name: &str,
        faction_name: &str,
        race_name: &str,
        position: Vec3,
        rotation: Quat,
        scale: Vec3,
    ) -> u32 {
        let Some(hull) = self.sof_data.hull_named(hull_name).cloned() else {
            return INVALID_PART_TAG;
        };

        let part_tag = self.object.part_data.get_unused_part_id();
        let effective_faction = if faction_name.is_empty() {
            self.object.part_data.faction.clone()
        } else {
            faction_name.to_owned()
        };
        let effective_race = if race_name.is_empty() {
            self.object.part_data.race.clone()
        } else {
            race_name.to_owned()
        };
        let damage_locators = hull
            .locator_sets
            .iter()
            .filter(|set| set.name == "damage")
            .flat_map(|set| set.locators.iter().map(|locator| locator.position))
            .collect();

        self.object.part_data.parts.push(EveModularPartData {
            part_tag,
            hull_name: hull.name,
            effective_faction,
            effective_race,
            transform: EveSOFDataTransform {
                position,
                rotation,
                scale,
            },
            local_bounding_sphere: hull.bounding_sphere,
            geometry_res_file_path: hull.geometry_res_file_path,
            is_skinned: hull.is_skinned,
            damage_locators,
            has_sound_emitters: !hull.sound_emitters.is_empty(),
        });
        part_tag
    }

    pub fn remove(&mut self, part_tag: u32) -> Result<(), ModularError> {
        let Some(index) = self
            .object
            .part_data
            .parts
            .iter()
            .position(|part| part.part_tag == part_tag)
        else {
            return Err(ModularError::KeyError);
        };
        self.object.part_data.parts.remove(index);
        Ok(())
    }

    pub fn set_transform(
        &mut self,
        part_tag: u32,
        position: Vec3,
        rotation: Quat,
        scale: Vec3,
    ) -> Result<(), ModularError> {
        let Some(part) = self.object.part_mut(part_tag) else {
            return Err(ModularError::KeyError);
        };
        part.transform = EveSOFDataTransform {
            position,
            rotation,
            scale,
        };
        Ok(())
    }

    pub fn get_position(&self, part_tag: u32) -> Result<Vec3, ModularError> {
        self.object
            .part(part_tag)
            .map(|part| part.transform.position)
            .ok_or(ModularError::KeyError)
    }

    pub fn get_rotation(&self, part_tag: u32) -> Result<Quat, ModularError> {
        self.object
            .part(part_tag)
            .map(|part| part.transform.rotation)
            .ok_or(ModularError::KeyError)
    }

    pub fn get_scale(&self, part_tag: u32) -> Result<Vec3, ModularError> {
        self.object
            .part(part_tag)
            .map(|part| part.transform.scale)
            .ok_or(ModularError::KeyError)
    }

    pub fn apply_bounds(&mut self) {
        self.object.recompute_bounds();
    }
}

impl Drop for EveModularObjectModifier<'_> {
    fn drop(&mut self) {
        self.object.recompute_bounds();
    }
}

#[derive(Resource, Reflect, Clone, Debug, Default, TrinityClass)]
#[reflect(Resource)]
#[trinity(
    module = "trinity",
    name = "EveSOF",
    legacy_header = "trinity/Eve/SpaceObjectFactory/EveSOF.h",
    kind = "resource"
)]
pub struct EveSOF {
    #[trinity(read, write, persist)]
    pub data: EveSOFData,
}

impl EveSOF {
    pub fn set_data(&mut self, data: EveSOFData) {
        self.data = data;
    }

    #[must_use]
    pub fn create_modular_object(
        &self,
        world: &mut World,
        faction_name: &str,
        race_name: &str,
    ) -> Entity {
        world
            .spawn(EveSpaceObject2 {
                part_data: EveChildPartData {
                    faction: faction_name.to_owned(),
                    race: race_name.to_owned(),
                    parts: Vec::new(),
                },
                ..Default::default()
            })
            .id()
    }

    pub fn modify_modular_object<'w>(
        &self,
        world: &'w mut World,
        entity: Entity,
    ) -> Option<EveModularObjectModifier<'w>> {
        let object = world.get_mut::<EveSpaceObject2>(entity)?;
        Some(EveModularObjectModifier {
            object,
            sof_data: self.data.clone(),
        })
    }

    #[must_use]
    pub fn create_modular_object_with_hull(
        &self,
        world: &mut World,
        hull_name: &str,
    ) -> Option<Entity> {
        let entity = self.create_modular_object(world, "", "");
        let part_tag = {
            let mut modifier = self.modify_modular_object(world, entity)?;
            modifier.add_hull(
                hull_name,
                "",
                "",
                Vec3::ZERO,
                Quat::IDENTITY,
                Vec3::ONE,
            )
        };
        if part_tag == INVALID_PART_TAG {
            let _ = world.despawn(entity);
            None
        } else {
            Some(entity)
        }
    }
}

#[allow(non_snake_case)]
#[must_use]
pub const fn GetInvalidPartTag() -> u32 {
    INVALID_PART_TAG
}

#[derive(Clone, Copy, Debug)]
struct ModularSphere {
    center: Vec3,
    radius: f32,
}

impl ModularSphere {
    #[must_use]
    fn include(self, other: Self) -> Self {
        let offset = other.center - self.center;
        let distance = offset.length();

        if self.radius >= distance + other.radius {
            return self;
        }
        if other.radius >= distance + self.radius {
            return other;
        }
        if distance == 0.0 {
            return Self {
                center: self.center,
                radius: self.radius.max(other.radius),
            };
        }

        let radius = (distance + self.radius + other.radius) * 0.5;
        let center = self.center + offset * ((radius - self.radius) / distance);
        Self { center, radius }
    }
}
