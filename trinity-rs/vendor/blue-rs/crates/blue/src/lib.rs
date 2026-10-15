use bevy::prelude::*;
pub use blueexposure;
#[derive(Resource,Reflect,Clone,Debug,Default)]#[reflect(Resource)]pub struct BlueLifecycle{pub frame:u64,pub shutdown_requested:bool}
#[derive(Default)]pub struct BluePlugin;
impl Plugin for BluePlugin{fn build(&self,app:&mut App){app.init_resource::<BlueLifecycle>().register_type::<BlueLifecycle>();}}
pub mod prelude{pub use blueexposure::prelude::*;pub use crate::{BlueLifecycle,BluePlugin};}
