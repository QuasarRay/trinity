use bevy::prelude::World;
use trinity_macros::trinity_test;
use trinity_rs::{EveSpaceObjectChild, TrinityHierarchyWorldExt};

#[trinity_test(
    legacy = "trinity/tests/Python/tests/test_childhierarchy.py::TestChildHierarchy::test_initValues",
    verification = "kani"
)]
fn child_init_values() {
    let mut world = World::new();
    let mesh = world.spawn_eve_child_mesh();
    let child = world.get::<EveSpaceObjectChild>(mesh).expect("child state");
    assert_eq!(None, child.owner());
    assert_eq!(None, child.parent());
    assert_eq!(0, child.part_tag);
}

#[trinity_test(
    legacy = "trinity/tests/Python/tests/test_childhierarchy.py::TestChildHierarchy::test_ownerAssignment",
    verification = "kani"
)]
fn owner_assignment() {
    let mut world = World::new();
    let ship = world.spawn_eve_ship2();
    let mesh = world.spawn_eve_child_mesh();
    assert!(world.attach_effect_child(ship, mesh));
    assert_eq!(
        Some(ship),
        world.get::<EveSpaceObjectChild>(mesh).unwrap().owner()
    );
}

#[trinity_test(
    legacy = "trinity/tests/Python/tests/test_childhierarchy.py::TestChildHierarchy::test_transitiveOwnerAssignment",
    verification = "kani"
)]
fn transitive_owner_assignment() {
    let mut world = World::new();
    let ship = world.spawn_eve_ship2();
    let container = world.spawn_eve_child_container();
    let mesh = world.spawn_eve_child_mesh();
    assert!(world.attach_effect_child(ship, container));
    assert!(world.attach_container_child(container, mesh));
    assert_eq!(
        Some(ship),
        world.get::<EveSpaceObjectChild>(mesh).unwrap().owner()
    );
}

#[trinity_test(
    legacy = "trinity/tests/Python/tests/test_childhierarchy.py::TestChildHierarchy::test_parentAssignment",
    verification = "kani"
)]
fn parent_assignment() {
    let mut world = World::new();
    let container = world.spawn_eve_child_container();
    let mesh = world.spawn_eve_child_mesh();
    assert!(world.attach_container_child(container, mesh));
    assert_eq!(
        Some(container),
        world.get::<EveSpaceObjectChild>(mesh).unwrap().parent()
    );
}

#[trinity_test(
    legacy = "trinity/tests/Python/tests/test_childhierarchy.py::TestChildHierarchy::test_ownerRemoval",
    verification = "kani"
)]
fn owner_removal() {
    let mut world = World::new();
    let ship = world.spawn_eve_ship2();
    let container = world.spawn_eve_child_container();
    let mesh = world.spawn_eve_child_mesh();
    world.attach_effect_child(ship, container);
    world.attach_container_child(container, mesh);
    assert!(world.detach_child(container));
    assert_eq!(
        None,
        world.get::<EveSpaceObjectChild>(container).unwrap().owner()
    );
    assert_eq!(
        None,
        world.get::<EveSpaceObjectChild>(mesh).unwrap().owner()
    );
}

#[trinity_test(
    legacy = "trinity/tests/Python/tests/test_childhierarchy.py::TestChildHierarchy::test_parentRemoval",
    verification = "kani"
)]
fn parent_removal() {
    let mut world = World::new();
    let container = world.spawn_eve_child_container();
    let mesh = world.spawn_eve_child_mesh();
    world.attach_container_child(container, mesh);
    assert!(world.detach_child(mesh));
    assert_eq!(
        None,
        world.get::<EveSpaceObjectChild>(mesh).unwrap().parent()
    );
}
