use bevy::prelude::{Quat, Vec3, Vec4, World};
use trinity_macros::trinity_test;
use trinity_rs::{
    EveSOF, EveSOFData, EveSOFDataFaction, EveSOFDataHull, EveSOFDataHullLocatorSet,
    EveSOFDataHullSoundEmitter, EveSOFDataRace, EveSOFDataTransform, EveSpaceObject2,
    GetInvalidPartTag, ModularError,
};

const FACTION: &str = "testfaction";
const RACE: &str = "testrace";

fn assert_f32_eq(expected: f32, actual: f32) {
    assert!(
        (expected - actual).abs() <= f32::EPSILON,
        "expected {expected}, got {actual}"
    );
}

fn sof() -> EveSOF {
    let damage_locator = EveSOFDataTransform {
        position: Vec3::ZERO,
        ..Default::default()
    };
    EveSOF {
        data: EveSOFData {
            hull: vec![
                EveSOFDataHull {
                    name: "static_hull".into(),
                    geometry_res_file_path: "res:/mygeo.cmf".into(),
                    bounding_sphere: Vec4::new(0.0, 0.0, 0.0, 50.0),
                    locator_sets: vec![EveSOFDataHullLocatorSet {
                        name: "damage".into(),
                        locators: vec![damage_locator],
                    }],
                    ..Default::default()
                },
                EveSOFDataHull {
                    name: "static_hull2".into(),
                    geometry_res_file_path: "res:/mygeo2.cmf".into(),
                    bounding_sphere: Vec4::new(0.0, 0.0, 0.0, 60.0),
                    ..Default::default()
                },
                EveSOFDataHull {
                    name: "anim_hull".into(),
                    geometry_res_file_path: "res:/mygeo.cmf".into(),
                    bounding_sphere: Vec4::new(0.0, 0.0, 0.0, 70.0),
                    is_skinned: true,
                    ..Default::default()
                },
                EveSOFDataHull {
                    name: "audio_hull".into(),
                    geometry_res_file_path: "res:/mygeo.cmf".into(),
                    bounding_sphere: Vec4::new(0.0, 0.0, 0.0, 50.0),
                    sound_emitters: vec![EveSOFDataHullSoundEmitter {
                        name: "engine_sound".into(),
                        position: Vec3::new(1.0, 2.0, 3.0),
                    }],
                    ..Default::default()
                },
            ],
            faction: vec![EveSOFDataFaction {
                name: FACTION.into(),
            }],
            race: vec![EveSOFDataRace { name: RACE.into() }],
            ..Default::default()
        },
    }
}

fn create_ship(sof: &EveSOF, world: &mut World) -> bevy::prelude::Entity {
    sof.create_modular_object(world, FACTION, RACE)
}

fn add_hull(
    sof: &EveSOF,
    world: &mut World,
    ship: bevy::prelude::Entity,
    name: &str,
    position: Vec3,
    rotation: Quat,
    scale: Vec3,
) -> u32 {
    let mut modifier = sof
        .modify_modular_object(world, ship)
        .expect("modular ship");
    modifier.add_hull(name, FACTION, RACE, position, rotation, scale)
}

#[trinity_test(
    legacy = "doc/source/eve/modular-assembly.rst::part-tags-stay-valid-across-sessions",
    verification = "kani"
)]
fn part_tags_are_stable_and_unique() {
    let sof = sof();
    let mut world = World::new();
    let ship = create_ship(&sof, &mut world);
    let first = add_hull(
        &sof,
        &mut world,
        ship,
        "static_hull",
        Vec3::ZERO,
        Quat::IDENTITY,
        Vec3::ONE,
    );
    let second = add_hull(
        &sof,
        &mut world,
        ship,
        "static_hull2",
        Vec3::ZERO,
        Quat::IDENTITY,
        Vec3::ONE,
    );
    assert_ne!(first, second);
}

#[trinity_test(
    legacy = "doc/source/eve/modular-assembly.rst::set-transform-persists-per-part",
    verification = "kani"
)]
fn set_transform_updates_selected_part() {
    let sof = sof();
    let mut world = World::new();
    let ship = create_ship(&sof, &mut world);
    let part = add_hull(
        &sof,
        &mut world,
        ship,
        "static_hull",
        Vec3::ZERO,
        Quat::IDENTITY,
        Vec3::ONE,
    );
    {
        let mut modifier = sof.modify_modular_object(&mut world, ship).unwrap();
        modifier
            .set_transform(
                part,
                Vec3::new(-30.0, 0.0, 0.0),
                Quat::IDENTITY,
                Vec3::ONE,
            )
            .unwrap();
    }
    assert_eq!(
        Vec3::new(-30.0, 0.0, 0.0),
        world.get::<EveSpaceObject2>(ship).unwrap().part(part).unwrap().transform.position
    );
}

#[trinity_test(
    legacy = "trinity/tests/Python/tests/test_modular.py::TestModular::test_addStaticHull",
    verification = "kani"
)]
fn add_static_hull_creates_one_static_mesh_group() {
    let sof = sof();
    let mut world = World::new();
    let ship = create_ship(&sof, &mut world);
    add_hull(&sof, &mut world, ship, "static_hull", Vec3::ZERO, Quat::IDENTITY, Vec3::ONE);
    let object = world.get::<EveSpaceObject2>(ship).unwrap();
    assert_eq!(1, object.static_mesh_count());
    assert_eq!(Some(1), object.static_instance_count(0));
}

#[trinity_test(
    legacy = "trinity/tests/Python/tests/test_modular.py::TestModular::test_addAnimatedHull",
    verification = "kani"
)]
fn add_animated_hull_is_not_instanced() {
    let sof = sof();
    let mut world = World::new();
    let ship = create_ship(&sof, &mut world);
    add_hull(&sof, &mut world, ship, "anim_hull", Vec3::ZERO, Quat::IDENTITY, Vec3::ONE);
    let object = world.get::<EveSpaceObject2>(ship).unwrap();
    assert_eq!(0, object.static_mesh_count());
    assert_eq!(1, object.animated_hull_count());
}

#[trinity_test(
    legacy = "trinity/tests/Python/tests/test_modular.py::TestModular::test_addTwoStaticHulls",
    verification = "kani"
)]
fn add_two_static_hulls_share_mesh_group() {
    let sof = sof();
    let mut world = World::new();
    let ship = create_ship(&sof, &mut world);
    add_hull(&sof, &mut world, ship, "static_hull", Vec3::ZERO, Quat::IDENTITY, Vec3::ONE);
    add_hull(&sof, &mut world, ship, "static_hull", Vec3::new(1.0, 2.0, 3.0), Quat::IDENTITY, Vec3::ONE);
    let object = world.get::<EveSpaceObject2>(ship).unwrap();
    assert_eq!(1, object.static_mesh_count());
    assert_eq!(Some(2), object.static_instance_count(0));
}

#[trinity_test(
    legacy = "trinity/tests/Python/tests/test_modular.py::TestModular::test_addTwoDifferentStaticHulls",
    verification = "kani"
)]
fn add_two_different_static_hulls_make_two_groups() {
    let sof = sof();
    let mut world = World::new();
    let ship = create_ship(&sof, &mut world);
    add_hull(&sof, &mut world, ship, "static_hull", Vec3::ZERO, Quat::IDENTITY, Vec3::ONE);
    add_hull(&sof, &mut world, ship, "static_hull2", Vec3::new(1.0, 2.0, 3.0), Quat::IDENTITY, Vec3::ONE);
    let object = world.get::<EveSpaceObject2>(ship).unwrap();
    assert_eq!(2, object.static_mesh_count());
    assert_eq!(Some(1), object.static_instance_count(0));
    assert_eq!(Some(1), object.static_instance_count(1));
}

#[trinity_test(
    legacy = "trinity/tests/Python/tests/test_modular.py::TestModular::test_addHullUnknownHullReturnsInvalidTag",
    verification = "kani"
)]
fn unknown_hull_returns_invalid_tag() {
    let sof = sof();
    let mut world = World::new();
    let ship = create_ship(&sof, &mut world);
    let tag = add_hull(&sof, &mut world, ship, "invalid_hull_name", Vec3::ZERO, Quat::IDENTITY, Vec3::ONE);
    assert_eq!(GetInvalidPartTag(), tag);
    assert_eq!(0, world.get::<EveSpaceObject2>(ship).unwrap().static_mesh_count());
}

#[trinity_test(
    legacy = "trinity/tests/Python/tests/test_modular.py::TestModular::test_partIdsAreUnique",
    verification = "kani"
)]
fn part_ids_are_unique() {
    let sof = sof();
    let mut world = World::new();
    let ship = create_ship(&sof, &mut world);
    let first = add_hull(&sof, &mut world, ship, "static_hull", Vec3::ZERO, Quat::IDENTITY, Vec3::ONE);
    let second = add_hull(&sof, &mut world, ship, "anim_hull", Vec3::ZERO, Quat::IDENTITY, Vec3::ONE);
    assert_ne!(first, second);
    assert_ne!(GetInvalidPartTag(), first);
    assert_ne!(GetInvalidPartTag(), second);
}

#[trinity_test(
    legacy = "trinity/tests/Python/tests/test_modular.py::TestModular::test_partIdRecycledAfterRemove",
    verification = "kani"
)]
fn highest_part_id_is_recycled_after_remove() {
    let sof = sof();
    let mut world = World::new();
    let ship = create_ship(&sof, &mut world);
    let first = add_hull(&sof, &mut world, ship, "static_hull", Vec3::new(20.0, 10.0, 0.0), Quat::IDENTITY, Vec3::ONE);
    {
        let mut modifier = sof.modify_modular_object(&mut world, ship).unwrap();
        modifier.remove(first).unwrap();
        let second = modifier.add_hull(
            "static_hull",
            FACTION,
            RACE,
            Vec3::new(10.0, 20.0, 0.0),
            Quat::IDENTITY,
            Vec3::ONE,
        );
        assert_eq!(first, second);
        assert_eq!(Ok(Vec3::new(10.0, 20.0, 0.0)), modifier.get_position(second));
    }
}

#[trinity_test(
    legacy = "trinity/tests/Python/tests/test_modular.py::TestModular::test_removeWrongID",
    verification = "kani"
)]
fn remove_wrong_id_returns_key_error() {
    let sof = sof();
    let mut world = World::new();
    let ship = create_ship(&sof, &mut world);
    add_hull(&sof, &mut world, ship, "anim_hull", Vec3::ZERO, Quat::IDENTITY, Vec3::ONE);
    let mut modifier = sof.modify_modular_object(&mut world, ship).unwrap();
    assert_eq!(Err(ModularError::KeyError), modifier.remove(5000));
}

#[trinity_test(
    legacy = "trinity/tests/Python/tests/test_modular.py::TestModular::test_setTransformRemovedPartRaises",
    verification = "kani"
)]
fn set_transform_removed_part_returns_key_error() {
    let sof = sof();
    let mut world = World::new();
    let ship = create_ship(&sof, &mut world);
    let part = add_hull(&sof, &mut world, ship, "static_hull", Vec3::ZERO, Quat::IDENTITY, Vec3::ONE);
    let mut modifier = sof.modify_modular_object(&mut world, ship).unwrap();
    modifier.remove(part).unwrap();
    assert_eq!(
        Err(ModularError::KeyError),
        modifier.set_transform(part, Vec3::ZERO, Quat::IDENTITY, Vec3::ONE)
    );
}

#[trinity_test(
    legacy = "trinity/tests/Python/tests/test_modular.py::TestModular::test_getRotationAndScale",
    verification = "kani"
)]
fn get_rotation_and_scale() {
    let sof = sof();
    let mut world = World::new();
    let ship = create_ship(&sof, &mut world);
    let rotation = Quat::from_xyzw(0.5, 0.5, 0.5, 0.5);
    let scale = Vec3::new(2.0, 4.0, 8.0);
    let part = add_hull(&sof, &mut world, ship, "static_hull", Vec3::ZERO, rotation, scale);
    let modifier = sof.modify_modular_object(&mut world, ship).unwrap();
    assert_eq!(Ok(rotation), modifier.get_rotation(part));
    assert_eq!(Ok(scale), modifier.get_scale(part));
}

#[trinity_test(
    legacy = "trinity/tests/Python/tests/test_modular.py::TestModular::test_getTransformUnknownPartRaises",
    verification = "kani"
)]
fn get_transform_unknown_part_returns_key_error() {
    let sof = sof();
    let mut world = World::new();
    let ship = create_ship(&sof, &mut world);
    let modifier = sof.modify_modular_object(&mut world, ship).unwrap();
    assert_eq!(Err(ModularError::KeyError), modifier.get_position(5000));
    assert_eq!(Err(ModularError::KeyError), modifier.get_rotation(5000));
    assert_eq!(Err(ModularError::KeyError), modifier.get_scale(5000));
}

#[trinity_test(
    legacy = "trinity/tests/Python/tests/test_modular.py::TestModular::test_addHullEmptyFactionRaceUsesDefaults",
    verification = "kani"
)]
fn empty_faction_and_race_use_seed_defaults() {
    let sof = sof();
    let mut world = World::new();
    let ship = create_ship(&sof, &mut world);
    let part = {
        let mut modifier = sof.modify_modular_object(&mut world, ship).unwrap();
        modifier.add_hull("static_hull", "", "", Vec3::ZERO, Quat::IDENTITY, Vec3::ONE)
    };
    let object = world.get::<EveSpaceObject2>(ship).unwrap();
    let part = object.part(part).unwrap();
    assert_eq!(FACTION, part.effective_faction);
    assert_eq!(RACE, part.effective_race);
}

#[trinity_test(
    legacy = "trinity/tests/Python/tests/test_modular.py::TestModular::test_setBoundingSphere",
    verification = "kani"
)]
fn set_bounding_sphere_from_one_part() {
    let sof = sof();
    let mut world = World::new();
    let ship = create_ship(&sof, &mut world);
    add_hull(&sof, &mut world, ship, "static_hull", Vec3::ZERO, Quat::IDENTITY, Vec3::ONE);
    let object = world.get::<EveSpaceObject2>(ship).unwrap();
    assert_f32_eq(50.0, object.bounding_sphere_radius);
    assert_eq!(Vec3::ZERO, object.bounding_sphere_center);
}

#[trinity_test(
    legacy = "trinity/tests/Python/tests/test_modular.py::TestModular::test_boundingSphereAccountsForTransforms",
    verification = "kani"
)]
fn bounding_sphere_accounts_for_translation() {
    let sof = sof();
    let mut world = World::new();
    let ship = create_ship(&sof, &mut world);
    add_hull(&sof, &mut world, ship, "static_hull", Vec3::new(30.0, 0.0, 0.0), Quat::IDENTITY, Vec3::ONE);
    let object = world.get::<EveSpaceObject2>(ship).unwrap();
    assert_f32_eq(50.0, object.bounding_sphere_radius);
    assert_eq!(Vec3::new(30.0, 0.0, 0.0), object.bounding_sphere_center);
}

#[trinity_test(
    legacy = "trinity/tests/Python/tests/test_modular.py::TestModular::test_boundingSphereEncapsulatesAllChildren",
    verification = "kani"
)]
fn bounding_sphere_encapsulates_all_children() {
    let sof = sof();
    let mut world = World::new();
    let ship = create_ship(&sof, &mut world);
    {
        let mut modifier = sof.modify_modular_object(&mut world, ship).unwrap();
        modifier.add_hull("static_hull", FACTION, RACE, Vec3::new(30.0, 0.0, 0.0), Quat::IDENTITY, Vec3::ONE);
        modifier.add_hull("static_hull", FACTION, RACE, Vec3::new(-30.0, 0.0, 0.0), Quat::IDENTITY, Vec3::ONE);
    }
    let object = world.get::<EveSpaceObject2>(ship).unwrap();
    assert_f32_eq(80.0, object.bounding_sphere_radius);
    assert_eq!(Vec3::ZERO, object.bounding_sphere_center);
}

#[trinity_test(
    legacy = "trinity/tests/Python/tests/test_modular.py::TestModular::test_boundingSphereUpdatesOnRemoval",
    verification = "kani"
)]
fn bounding_sphere_updates_on_removal() {
    let sof = sof();
    let mut world = World::new();
    let ship = create_ship(&sof, &mut world);
    let positive = {
        let mut modifier = sof.modify_modular_object(&mut world, ship).unwrap();
        let tag = modifier.add_hull("static_hull", FACTION, RACE, Vec3::new(30.0, 0.0, 0.0), Quat::IDENTITY, Vec3::ONE);
        modifier.add_hull("static_hull", FACTION, RACE, Vec3::new(-30.0, 0.0, 0.0), Quat::IDENTITY, Vec3::ONE);
        tag
    };
    {
        let mut modifier = sof.modify_modular_object(&mut world, ship).unwrap();
        modifier.remove(positive).unwrap();
    }
    let object = world.get::<EveSpaceObject2>(ship).unwrap();
    assert_f32_eq(50.0, object.bounding_sphere_radius);
    assert_eq!(Vec3::new(-30.0, 0.0, 0.0), object.bounding_sphere_center);
}

#[trinity_test(
    legacy = "trinity/tests/Python/tests/test_modular.py::TestModular::test_boundingSphereUpdatesOnSetTransform",
    verification = "kani"
)]
fn bounding_sphere_updates_on_set_transform() {
    let sof = sof();
    let mut world = World::new();
    let ship = create_ship(&sof, &mut world);
    let part = add_hull(&sof, &mut world, ship, "static_hull", Vec3::new(30.0, 0.0, 0.0), Quat::IDENTITY, Vec3::ONE);
    {
        let mut modifier = sof.modify_modular_object(&mut world, ship).unwrap();
        modifier.set_transform(part, Vec3::new(-30.0, 0.0, 0.0), Quat::IDENTITY, Vec3::ONE).unwrap();
    }
    assert_eq!(
        Vec3::new(-30.0, 0.0, 0.0),
        world.get::<EveSpaceObject2>(ship).unwrap().bounding_sphere_center
    );
}

#[trinity_test(
    legacy = "trinity/tests/Python/tests/test_modular.py::TestModular::test_boundingSphereScalesWithSetTransform",
    verification = "kani"
)]
fn bounding_sphere_scales_with_transform() {
    let sof = sof();
    let mut world = World::new();
    let ship = create_ship(&sof, &mut world);
    let part = add_hull(&sof, &mut world, ship, "static_hull", Vec3::ZERO, Quat::IDENTITY, Vec3::ONE);
    {
        let mut modifier = sof.modify_modular_object(&mut world, ship).unwrap();
        modifier.set_transform(part, Vec3::ZERO, Quat::IDENTITY, Vec3::splat(2.0)).unwrap();
    }
    assert_f32_eq(
        100.0,
        world.get::<EveSpaceObject2>(ship).unwrap().bounding_sphere_radius,
    );
}

#[trinity_test(
    legacy = "trinity/tests/Python/tests/test_modular.py::TestModular::test_boundingSphereConcentricParts",
    verification = "kani"
)]
fn bounding_sphere_concentric_parts_uses_largest() {
    let sof = sof();
    let mut world = World::new();
    let ship = create_ship(&sof, &mut world);
    {
        let mut modifier = sof.modify_modular_object(&mut world, ship).unwrap();
        modifier.add_hull("static_hull", FACTION, RACE, Vec3::ZERO, Quat::IDENTITY, Vec3::ONE);
        modifier.add_hull("static_hull2", FACTION, RACE, Vec3::ZERO, Quat::IDENTITY, Vec3::ONE);
    }
    let object = world.get::<EveSpaceObject2>(ship).unwrap();
    assert_f32_eq(60.0, object.bounding_sphere_radius);
    assert_eq!(Vec3::ZERO, object.bounding_sphere_center);
}

#[trinity_test(
    legacy = "trinity/tests/Python/tests/test_modular.py::TestModular::test_shapeEllipsoidEncapsulatesAllParts",
    verification = "kani"
)]
fn shape_ellipsoid_center_tracks_parts() {
    let sof = sof();
    let mut world = World::new();
    let ship = create_ship(&sof, &mut world);
    add_hull(&sof, &mut world, ship, "static_hull", Vec3::new(30.0, 0.0, 0.0), Quat::IDENTITY, Vec3::ONE);
    assert_eq!(
        Vec3::new(30.0, 0.0, 0.0),
        world.get::<EveSpaceObject2>(ship).unwrap().shape_ellipsoid_center
    );
    add_hull(&sof, &mut world, ship, "static_hull", Vec3::new(-30.0, 0.0, 0.0), Quat::IDENTITY, Vec3::ONE);
    assert_eq!(
        Vec3::ZERO,
        world.get::<EveSpaceObject2>(ship).unwrap().shape_ellipsoid_center
    );
}

#[trinity_test(
    legacy = "trinity/tests/Python/tests/test_modular.py::TestModular::test_shapeEllipsoidEmptyAfterRemovingAllParts",
    verification = "kani"
)]
fn shape_ellipsoid_empty_after_removing_all_parts() {
    let sof = sof();
    let mut world = World::new();
    let ship = create_ship(&sof, &mut world);
    let part = add_hull(&sof, &mut world, ship, "static_hull", Vec3::new(30.0, 0.0, 0.0), Quat::IDENTITY, Vec3::ONE);
    {
        let mut modifier = sof.modify_modular_object(&mut world, ship).unwrap();
        modifier.remove(part).unwrap();
    }
    assert_eq!(
        Vec3::ZERO,
        world.get::<EveSpaceObject2>(ship).unwrap().shape_ellipsoid_center
    );
}

#[trinity_test(
    legacy = "trinity/tests/Python/tests/test_modular.py::TestModular::test_addsLocators",
    verification = "kani"
)]
fn adds_damage_locators() {
    let sof = sof();
    let mut world = World::new();
    let ship = create_ship(&sof, &mut world);
    add_hull(&sof, &mut world, ship, "static_hull", Vec3::new(30.0, 0.0, 0.0), Quat::IDENTITY, Vec3::ONE);
    assert_eq!(1, world.get::<EveSpaceObject2>(ship).unwrap().damage_locator_count());
}

#[trinity_test(
    legacy = "trinity/tests/Python/tests/test_modular.py::TestModular::test_locatorsInheritTransform",
    verification = "kani"
)]
fn locators_inherit_transform() {
    let sof = sof();
    let mut world = World::new();
    let ship = create_ship(&sof, &mut world);
    add_hull(&sof, &mut world, ship, "static_hull", Vec3::new(30.0, 0.0, 0.0), Quat::IDENTITY, Vec3::ONE);
    assert_eq!(
        Some(Vec3::new(30.0, 0.0, 0.0)),
        world.get::<EveSpaceObject2>(ship).unwrap().damage_locator(0)
    );
}

#[trinity_test(
    legacy = "trinity/tests/Python/tests/test_modular.py::TestModular::test_addsLocatorsMerged",
    verification = "kani"
)]
fn damage_locators_merge_across_parts() {
    let sof = sof();
    let mut world = World::new();
    let ship = create_ship(&sof, &mut world);
    add_hull(&sof, &mut world, ship, "static_hull", Vec3::new(30.0, 0.0, 0.0), Quat::IDENTITY, Vec3::ONE);
    add_hull(&sof, &mut world, ship, "static_hull", Vec3::new(-30.0, 0.0, 0.0), Quat::IDENTITY, Vec3::ONE);
    assert_eq!(2, world.get::<EveSpaceObject2>(ship).unwrap().damage_locator_count());
}

#[trinity_test(
    legacy = "trinity/tests/Python/tests/test_modular.py::TestModular::test_moveLocators",
    verification = "kani"
)]
fn moving_part_moves_locators() {
    let sof = sof();
    let mut world = World::new();
    let ship = create_ship(&sof, &mut world);
    let part = add_hull(&sof, &mut world, ship, "static_hull", Vec3::new(10.0, 20.0, 0.0), Quat::IDENTITY, Vec3::ONE);
    {
        let mut modifier = sof.modify_modular_object(&mut world, ship).unwrap();
        modifier.set_transform(part, Vec3::new(1.0, 2.0, 3.0), Quat::IDENTITY, Vec3::ONE).unwrap();
    }
    assert_eq!(
        Some(Vec3::new(1.0, 2.0, 3.0)),
        world.get::<EveSpaceObject2>(ship).unwrap().damage_locator(0)
    );
}

#[trinity_test(
    legacy = "trinity/tests/Python/tests/test_modular.py::TestModular::test_getTransformReflectsSetTransform",
    verification = "kani"
)]
fn getters_reflect_set_transform() {
    let sof = sof();
    let mut world = World::new();
    let ship = create_ship(&sof, &mut world);
    let part = add_hull(&sof, &mut world, ship, "static_hull", Vec3::ZERO, Quat::IDENTITY, Vec3::ONE);
    let rotation = Quat::from_xyzw(0.5, 0.5, 0.5, 0.5);
    let scale = Vec3::new(2.0, 4.0, 8.0);
    let mut modifier = sof.modify_modular_object(&mut world, ship).unwrap();
    modifier.set_transform(part, Vec3::new(1.0, 2.0, 3.0), rotation, scale).unwrap();
    assert_eq!(Ok(Vec3::new(1.0, 2.0, 3.0)), modifier.get_position(part));
    assert_eq!(Ok(rotation), modifier.get_rotation(part));
    assert_eq!(Ok(scale), modifier.get_scale(part));
}

#[trinity_test(
    legacy = "trinity/tests/Python/tests/test_modular.py::TestModular::test_partDataSurvivesModifierReopen",
    verification = "kani"
)]
fn part_data_survives_modifier_reopen() {
    let sof = sof();
    let mut world = World::new();
    let ship = create_ship(&sof, &mut world);
    let part = add_hull(&sof, &mut world, ship, "static_hull", Vec3::new(10.0, 20.0, 30.0), Quat::IDENTITY, Vec3::ONE);
    let modifier = sof.modify_modular_object(&mut world, ship).unwrap();
    assert_eq!(Ok(Vec3::new(10.0, 20.0, 30.0)), modifier.get_position(part));
}

#[trinity_test(
    legacy = "trinity/tests/Python/tests/test_modular.py::TestModular::test_doubleRemoveRaises",
    verification = "kani"
)]
fn double_remove_returns_key_error() {
    let sof = sof();
    let mut world = World::new();
    let ship = create_ship(&sof, &mut world);
    let part = add_hull(&sof, &mut world, ship, "static_hull", Vec3::ZERO, Quat::IDENTITY, Vec3::ONE);
    let mut modifier = sof.modify_modular_object(&mut world, ship).unwrap();
    modifier.remove(part).unwrap();
    assert_eq!(Err(ModularError::KeyError), modifier.remove(part));
}

#[trinity_test(
    legacy = "trinity/tests/Python/tests/test_modular.py::TestModular::test_removeInvalidPartTagRaises",
    verification = "kani"
)]
fn remove_invalid_tag_returns_key_error() {
    let sof = sof();
    let mut world = World::new();
    let ship = create_ship(&sof, &mut world);
    add_hull(&sof, &mut world, ship, "static_hull", Vec3::ZERO, Quat::IDENTITY, Vec3::ONE);
    let mut modifier = sof.modify_modular_object(&mut world, ship).unwrap();
    assert_eq!(Err(ModularError::KeyError), modifier.remove(GetInvalidPartTag()));
}

#[trinity_test(
    legacy = "trinity/tests/Python/tests/test_modular.py::TestModular::test_audioHullCreatesPlacementContainer",
    verification = "kani"
)]
fn audio_hull_creates_logical_container() {
    let sof = sof();
    let mut world = World::new();
    let ship = create_ship(&sof, &mut world);
    add_hull(&sof, &mut world, ship, "audio_hull", Vec3::ZERO, Quat::IDENTITY, Vec3::ONE);
    let object = world.get::<EveSpaceObject2>(ship).unwrap();
    assert_eq!(1, object.audio_container_count());
    assert_eq!(vec!["audio_hull"], object.audio_container_names());
}

#[trinity_test(
    legacy = "trinity/tests/Python/tests/test_modular.py::TestModular::test_staticHullWithoutEmittersHasNoContainer",
    verification = "kani"
)]
fn static_hull_without_emitters_has_no_container() {
    let sof = sof();
    let mut world = World::new();
    let ship = create_ship(&sof, &mut world);
    add_hull(&sof, &mut world, ship, "static_hull", Vec3::ZERO, Quat::IDENTITY, Vec3::ONE);
    assert_eq!(0, world.get::<EveSpaceObject2>(ship).unwrap().audio_container_count());
}

#[trinity_test(
    legacy = "trinity/tests/Python/tests/test_modular.py::TestModular::test_audioHullContainerCarriesPartTag",
    verification = "kani"
)]
fn audio_container_carries_part_tag() {
    let sof = sof();
    let mut world = World::new();
    let ship = create_ship(&sof, &mut world);
    let tag = add_hull(&sof, &mut world, ship, "audio_hull", Vec3::ZERO, Quat::IDENTITY, Vec3::ONE);
    assert_eq!(
        vec![tag],
        world.get::<EveSpaceObject2>(ship).unwrap().audio_container_part_tags()
    );
}

#[trinity_test(
    legacy = "trinity/tests/Python/tests/test_modular.py::TestModular::test_removeAudioHullRemovesContainer",
    verification = "kani"
)]
fn removing_audio_hull_removes_logical_container() {
    let sof = sof();
    let mut world = World::new();
    let ship = create_ship(&sof, &mut world);
    let tag = add_hull(&sof, &mut world, ship, "audio_hull", Vec3::ZERO, Quat::IDENTITY, Vec3::ONE);
    {
        let mut modifier = sof.modify_modular_object(&mut world, ship).unwrap();
        modifier.remove(tag).unwrap();
    }
    assert_eq!(0, world.get::<EveSpaceObject2>(ship).unwrap().audio_container_count());
}

#[trinity_test(
    legacy = "trinity/tests/Python/tests/test_modular.py::TestModular::test_removeOneOfTwoInstances",
    verification = "kani"
)]
fn remove_one_of_two_instances_keeps_one() {
    let sof = sof();
    let mut world = World::new();
    let ship = create_ship(&sof, &mut world);
    add_hull(&sof, &mut world, ship, "static_hull", Vec3::ZERO, Quat::IDENTITY, Vec3::ONE);
    let second = add_hull(&sof, &mut world, ship, "static_hull", Vec3::ONE, Quat::IDENTITY, Vec3::ONE);
    {
        let mut modifier = sof.modify_modular_object(&mut world, ship).unwrap();
        modifier.remove(second).unwrap();
    }
    assert_eq!(
        Some(1),
        world.get::<EveSpaceObject2>(ship).unwrap().static_instance_count(0)
    );
}

#[trinity_test(
    legacy = "trinity/tests/Python/tests/test_modular.py::TestModular::test_removeMiddleInstanceKeepsOthers",
    verification = "kani"
)]
fn remove_middle_instance_keeps_others() {
    let sof = sof();
    let mut world = World::new();
    let ship = create_ship(&sof, &mut world);
    add_hull(&sof, &mut world, ship, "static_hull", Vec3::ZERO, Quat::IDENTITY, Vec3::ONE);
    let middle = add_hull(&sof, &mut world, ship, "static_hull", Vec3::X, Quat::IDENTITY, Vec3::ONE);
    add_hull(&sof, &mut world, ship, "static_hull", Vec3::X * 2.0, Quat::IDENTITY, Vec3::ONE);
    {
        let mut modifier = sof.modify_modular_object(&mut world, ship).unwrap();
        modifier.remove(middle).unwrap();
    }
    assert_eq!(
        Some(2),
        world.get::<EveSpaceObject2>(ship).unwrap().static_instance_count(0)
    );
}

#[trinity_test(
    legacy = "trinity/tests/Python/tests/test_modular.py::TestModular::test_removeFirstOfTwoDifferentHullsKeepsSecond",
    verification = "kani"
)]
fn removing_first_different_hull_keeps_second_group() {
    let sof = sof();
    let mut world = World::new();
    let ship = create_ship(&sof, &mut world);
    let first = add_hull(&sof, &mut world, ship, "static_hull", Vec3::ZERO, Quat::IDENTITY, Vec3::ONE);
    add_hull(&sof, &mut world, ship, "static_hull2", Vec3::X, Quat::IDENTITY, Vec3::ONE);
    {
        let mut modifier = sof.modify_modular_object(&mut world, ship).unwrap();
        modifier.remove(first).unwrap();
    }
    let object = world.get::<EveSpaceObject2>(ship).unwrap();
    assert_eq!(1, object.static_mesh_count());
    assert_eq!(Some("res:/mygeo2.cmf".to_owned()), object.static_mesh_path(0));
    assert_eq!(Some(1), object.static_instance_count(0));
}

#[trinity_test(
    legacy = "trinity/tests/Python/tests/test_modular.py::TestModular::test_removeAndReaddIntoSharedMesh",
    verification = "kani"
)]
fn remove_and_readd_into_shared_mesh() {
    let sof = sof();
    let mut world = World::new();
    let ship = create_ship(&sof, &mut world);
    let first = add_hull(&sof, &mut world, ship, "static_hull", Vec3::ZERO, Quat::IDENTITY, Vec3::ONE);
    add_hull(&sof, &mut world, ship, "static_hull", Vec3::X, Quat::IDENTITY, Vec3::ONE);
    add_hull(&sof, &mut world, ship, "static_hull", Vec3::X * 2.0, Quat::IDENTITY, Vec3::ONE);
    {
        let mut modifier = sof.modify_modular_object(&mut world, ship).unwrap();
        modifier.remove(first).unwrap();
        let readded = modifier.add_hull("static_hull", FACTION, RACE, Vec3::X * 3.0, Quat::IDENTITY, Vec3::ONE);
        assert_ne!(GetInvalidPartTag(), readded);
    }
    let object = world.get::<EveSpaceObject2>(ship).unwrap();
    assert_eq!(1, object.static_mesh_count());
    assert_eq!(Some(3), object.static_instance_count(0));
}

#[trinity_test(
    legacy = "trinity/tests/Python/tests/test_modular.py::TestModifyExistingObject::test_modifyObjectWithoutPartDataSurvivesReopen",
    verification = "kani"
)]
fn modifying_existing_object_survives_reopen() {
    let sof = sof();
    let mut world = World::new();
    let ship = world.spawn(EveSpaceObject2::default()).id();
    let part = {
        let mut modifier = sof.modify_modular_object(&mut world, ship).unwrap();
        modifier.add_hull(
            "static_hull",
            FACTION,
            RACE,
            Vec3::new(1.0, 2.0, 3.0),
            Quat::IDENTITY,
            Vec3::ONE,
        )
    };
    let modifier = sof.modify_modular_object(&mut world, ship).unwrap();
    assert_eq!(Ok(Vec3::new(1.0, 2.0, 3.0)), modifier.get_position(part));
}
