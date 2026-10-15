use bevy::prelude::{Vec3, Vec4, World};
use trinity_macros::trinity_test;
use trinity_rs::{EveSOF, EveSOFData, EveSOFDataHull, EveSOFDataTransform, EveSpaceObject2};

fn sof() -> EveSOF {
    EveSOF { data: EveSOFData { hull: vec![EveSOFDataHull { name: "core".to_owned(), bounding_sphere: Vec4::new(0.0, 0.0, 0.0, 50.0), ..Default::default() }, EveSOFDataHull { name: "wing".to_owned(), bounding_sphere: Vec4::new(0.0, 0.0, 0.0, 10.0), ..Default::default() }], ..Default::default() } }
}

#[trinity_test(legacy = "doc/source/eve/modular-assembly.rst::part-tags-stay-valid-across-sessions", verification = "kani")]
fn part_tags_are_stable_and_unique() {
    let sof = sof();
    let mut world = World::new();
    let entity = sof.create_modular_object(&mut world, "core").expect("core hull");
    let wing = sof.add_hull(&mut world, entity, "wing").expect("wing hull");
    let object = world.get::<EveSpaceObject2>(entity).unwrap();
    assert_eq!(2, object.parts.len());
    assert_ne!(object.parts[0].part_tag, wing);
}

#[trinity_test(legacy = "doc/source/eve/modular-assembly.rst::set-transform-persists-per-part", verification = "kani")]
fn set_transform_updates_selected_part() {
    let sof = sof();
    let mut world = World::new();
    let entity = sof.create_modular_object(&mut world, "core").unwrap();
    let tag = world.get::<EveSpaceObject2>(entity).unwrap().parts[0].part_tag;
    let transform = EveSOFDataTransform { position: Vec3::new(-30.0, 0.0, 0.0), ..Default::default() };
    assert!(world.get_mut::<EveSpaceObject2>(entity).unwrap().set_transform(tag, transform));
    assert_eq!(transform, world.get::<EveSpaceObject2>(entity).unwrap().parts[0].transform);
}
