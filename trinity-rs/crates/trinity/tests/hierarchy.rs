use bevy::prelude::World;
use trinity_macros::trinity_test;
use trinity_rs::{EveSpaceObjectChild, TrinityHierarchyWorldExt};
#[trinity_test(legacy = "trinity/tests/Python/tests/test_childhierarchy.py::TestChildHierarchy::test_initValues", verification = "kani")]
fn child_init_values() { let mut w = World::new(); let m = w.spawn_eve_child_mesh(); let c = w.get::<EveSpaceObjectChild>(m).unwrap(); assert_eq!(None,c.owner()); assert_eq!(None,c.parent()); assert_eq!(0,c.part_tag); }
#[trinity_test(legacy = "trinity/tests/Python/tests/test_childhierarchy.py::TestChildHierarchy::test_ownerAssignment", verification = "kani")]
fn owner_assignment() { let mut w=World::new(); let s=w.spawn_eve_ship2(); let m=w.spawn_eve_child_mesh(); assert!(w.attach_effect_child(s,m)); assert_eq!(Some(s),w.get::<EveSpaceObjectChild>(m).unwrap().owner()); }
#[trinity_test(legacy = "trinity/tests/Python/tests/test_childhierarchy.py::TestChildHierarchy::test_transitiveOwnerAssignment", verification = "kani")]
fn transitive_owner_assignment() { let mut w=World::new(); let s=w.spawn_eve_ship2(); let c=w.spawn_eve_child_container(); let m=w.spawn_eve_child_mesh(); assert!(w.attach_effect_child(s,c)); assert!(w.attach_container_child(c,m)); assert_eq!(Some(s),w.get::<EveSpaceObjectChild>(m).unwrap().owner()); }
#[trinity_test(legacy = "trinity/tests/Python/tests/test_childhierarchy.py::TestChildHierarchy::test_parentAssignment", verification = "kani")]
fn parent_assignment() { let mut w=World::new(); let c=w.spawn_eve_child_container(); let m=w.spawn_eve_child_mesh(); assert!(w.attach_container_child(c,m)); assert_eq!(Some(c),w.get::<EveSpaceObjectChild>(m).unwrap().parent()); }
#[trinity_test(legacy = "trinity/tests/Python/tests/test_childhierarchy.py::TestChildHierarchy::test_ownerRemoval", verification = "kani")]
fn owner_removal() { let mut w=World::new(); let s=w.spawn_eve_ship2(); let c=w.spawn_eve_child_container(); let m=w.spawn_eve_child_mesh(); w.attach_effect_child(s,c); w.attach_container_child(c,m); assert!(w.detach_child(c)); assert_eq!(None,w.get::<EveSpaceObjectChild>(c).unwrap().owner()); assert_eq!(None,w.get::<EveSpaceObjectChild>(m).unwrap().owner()); }
#[trinity_test(legacy = "trinity/tests/Python/tests/test_childhierarchy.py::TestChildHierarchy::test_parentRemoval", verification = "kani")]
fn parent_removal() { let mut w=World::new(); let c=w.spawn_eve_child_container(); let m=w.spawn_eve_child_mesh(); w.attach_container_child(c,m); assert!(w.detach_child(m)); assert_eq!(None,w.get::<EveSpaceObjectChild>(m).unwrap().parent()); }
