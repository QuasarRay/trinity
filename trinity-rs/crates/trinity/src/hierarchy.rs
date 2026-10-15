use bevy::prelude::*;
use delegate::delegate;
use trinity_macros::TrinityClass;

#[derive(Component, Reflect, Clone, Copy, Debug, Default, Eq, PartialEq, TrinityClass)]
#[reflect(Component)]
#[trinity(module = "trinity", name = "EveSpaceObjectChild", legacy_header = "trinity/EveSpaceObjectChild.h", kind = "component")]
pub struct EveSpaceObjectChild {
    #[trinity(read, persist)]
    owner: Option<Entity>,
    #[trinity(read, persist)]
    parent: Option<Entity>,
    #[trinity(read, write, persist)]
    pub part_tag: u32,
}

impl EveSpaceObjectChild {
    #[must_use]
    pub const fn owner(&self) -> Option<Entity> { self.owner }
    #[must_use]
    pub const fn parent(&self) -> Option<Entity> { self.parent }

    pub(crate) fn set_owner(&mut self, owner: Option<Entity>) { self.owner = owner; }
    pub(crate) fn set_parent(&mut self, parent: Option<Entity>) { self.parent = parent; }
}

#[derive(Component, Reflect, Clone, Debug, Default, TrinityClass)]
#[reflect(Component)]
#[trinity(module = "trinity", name = "EveChildMesh", legacy_header = "trinity/EveSpaceObjectChild.h", kind = "component")]
pub struct EveChildMesh;

#[derive(Component, Reflect, Clone, Debug, Default, TrinityClass)]
#[reflect(Component)]
#[trinity(module = "trinity", name = "EveChildContainer", legacy_header = "trinity/EveSpaceObjectChild.h", kind = "component")]
pub struct EveChildContainer;

#[derive(Component, Reflect, Clone, Debug, Default, TrinityClass)]
#[reflect(Component)]
#[trinity(module = "trinity", name = "EveShip2", legacy_header = "trinity/EveShip2.h", kind = "component")]
pub struct EveShip2;

#[derive(Component, Reflect, Clone, Debug, Default)]
#[reflect(Component)]
pub struct EveChildRelations {
    pub children: Vec<Entity>,
}

impl EveChildRelations {
    delegate! {
        to self.children {
            pub fn len(&self) -> usize;
            pub fn is_empty(&self) -> bool;
        }
    }
}

pub trait TrinityHierarchyWorldExt {
    fn spawn_eve_ship2(&mut self) -> Entity;
    fn spawn_eve_child_mesh(&mut self) -> Entity;
    fn spawn_eve_child_container(&mut self) -> Entity;
    fn attach_effect_child(&mut self, owner: Entity, child: Entity) -> bool;
    fn attach_container_child(&mut self, container: Entity, child: Entity) -> bool;
    fn detach_child(&mut self, child: Entity) -> bool;
}

impl TrinityHierarchyWorldExt for World {
    fn spawn_eve_ship2(&mut self) -> Entity {
        self.spawn((EveShip2, EveChildRelations::default())).id()
    }

    fn spawn_eve_child_mesh(&mut self) -> Entity {
        self.spawn((EveChildMesh, EveSpaceObjectChild::default())).id()
    }

    fn spawn_eve_child_container(&mut self) -> Entity {
        self.spawn((
            EveChildContainer,
            EveSpaceObjectChild::default(),
            EveChildRelations::default(),
        ))
        .id()
    }

    fn attach_effect_child(&mut self, owner: Entity, child: Entity) -> bool {
        if self.get::<EveShip2>(owner).is_none() || self.get::<EveSpaceObjectChild>(child).is_none() {
            return false;
        }
        push_unique_child(self, owner, child);
        set_parent(self, child, None);
        propagate_owner(self, child, Some(owner));
        true
    }

    fn attach_container_child(&mut self, container: Entity, child: Entity) -> bool {
        let Some(container_state) = self.get::<EveSpaceObjectChild>(container).copied() else {
            return false;
        };
        if self.get::<EveChildContainer>(container).is_none() || self.get::<EveSpaceObjectChild>(child).is_none() {
            return false;
        }
        push_unique_child(self, container, child);
        set_parent(self, child, Some(container));
        propagate_owner(self, child, container_state.owner());
        true
    }

    fn detach_child(&mut self, child: Entity) -> bool {
        let Some(state) = self.get::<EveSpaceObjectChild>(child).copied() else {
            return false;
        };
        if let Some(parent) = state.parent().or(state.owner()) {
            if let Some(mut links) = self.get_mut::<EveChildRelations>(parent) {
                links.children.retain(|entity| *entity != child);
            }
        }
        set_parent(self, child, None);
        propagate_owner(self, child, None);
        true
    }
}

fn push_unique_child(world: &mut World, parent: Entity, child: Entity) {
    if world.get::<EveChildRelations>(parent).is_none() {
        world.entity_mut(parent).insert(EveChildRelations::default());
    }
    let mut links = world
        .get_mut::<EveChildRelations>(parent)
        .expect("relations inserted above");
    if !links.children.contains(&child) {
        links.children.push(child);
    }
}

fn set_parent(world: &mut World, entity: Entity, parent: Option<Entity>) {
    if let Some(mut state) = world.get_mut::<EveSpaceObjectChild>(entity) {
        state.set_parent(parent);
    }
}

fn propagate_owner(world: &mut World, root: Entity, owner: Option<Entity>) {
    let descendants = collect_descendants(world, root);
    if let Some(mut state) = world.get_mut::<EveSpaceObjectChild>(root) {
        state.set_owner(owner);
    }
    for entity in descendants {
        if let Some(mut state) = world.get_mut::<EveSpaceObjectChild>(entity) {
            state.set_owner(owner);
        }
    }
}

fn collect_descendants(world: &World, root: Entity) -> Vec<Entity> {
    let mut result = Vec::new();
    let mut stack = world
        .get::<EveChildRelations>(root)
        .map_or_else(Vec::new, |links| links.children.clone());
    while let Some(entity) = stack.pop() {
        result.push(entity);
        if let Some(links) = world.get::<EveChildRelations>(entity) {
            stack.extend(links.children.iter().copied());
        }
    }
    result
}
