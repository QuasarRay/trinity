use bevy::prelude::{Vec3, Vec4, World};
use trinity_macros::trinity_test;
use trinity_rs::{EveSOF,EveSOFData,EveSOFDataHull,EveSOFDataTransform,EveSpaceObject2};
fn sof()->EveSOF{EveSOF{data:EveSOFData{hull:vec![EveSOFDataHull{name:"core".into(),bounding_sphere:Vec4::new(0.,0.,0.,50.),..Default::default()},EveSOFDataHull{name:"wing".into(),bounding_sphere:Vec4::new(0.,0.,0.,10.),..Default::default()}],..Default::default()}}}
#[trinity_test(legacy = "doc/source/eve/modular-assembly.rst::part-tags-stay-valid-across-sessions", verification = "kani")]
fn part_tags_are_stable_and_unique(){let s=sof();let mut w=World::new();let e=s.create_modular_object(&mut w,"core").unwrap();let wing=s.add_hull(&mut w,e,"wing").unwrap();let o=w.get::<EveSpaceObject2>(e).unwrap();assert_eq!(2,o.parts.len());assert_ne!(o.parts[0].part_tag,wing);}
#[trinity_test(legacy = "doc/source/eve/modular-assembly.rst::set-transform-persists-per-part", verification = "kani")]
fn set_transform_updates_selected_part(){let s=sof();let mut w=World::new();let e=s.create_modular_object(&mut w,"core").unwrap();let tag=w.get::<EveSpaceObject2>(e).unwrap().parts[0].part_tag;let t=EveSOFDataTransform{position:Vec3::new(-30.,0.,0.),..Default::default()};assert!(w.get_mut::<EveSpaceObject2>(e).unwrap().set_transform(tag,t));assert_eq!(t,w.get::<EveSpaceObject2>(e).unwrap().parts[0].transform);}
