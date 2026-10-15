use bevy::prelude::*;
use trinity_macros::TrinityClass;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, Reflect)]
pub enum Tr2PPExecutionSlot { #[default] BeforeUpscaling, AfterTonemap }
#[derive(Clone, Debug, Default, Eq, PartialEq, Reflect)]
pub struct Tr2PPParameter { pub name: String, pub value_bits: u32, pub blendable: bool }
#[derive(Component, Reflect, Clone, Debug, Default, TrinityClass)]
#[reflect(Component)]
#[trinity(module = "trinity", name = "Tr2PPGenericEffect", legacy_header = "doc/source/eve/generic-pp-effects.rst", kind = "component")]
pub struct Tr2PPGenericEffect {
    #[trinity(read, write, persist)] pub effect: String,
    #[trinity(read, write, persist)] pub quality: u32,
    #[trinity(read, write, persist)] pub execution_slot: Tr2PPExecutionSlot,
    #[trinity(read, write, persist)] pub order: i32,
    #[trinity(read, write, persist)] pub parameters: Vec<Tr2PPParameter>,
}
impl Tr2PPGenericEffect {
    #[must_use] pub fn grouping_key(&self) -> (String, u32, Tr2PPExecutionSlot, Vec<(String, u32)>) {
        let mut non_blendable = self.parameters.iter().filter(|p| !p.blendable).map(|p| (p.name.clone(), p.value_bits)).collect::<Vec<_>>();
        non_blendable.sort_by(|l, r| l.0.cmp(&r.0));
        (self.effect.clone(), self.quality, self.execution_slot, non_blendable)
    }
    #[must_use] pub fn can_group_with(&self, other: &Self) -> bool { self.grouping_key() == other.grouping_key() }
}
