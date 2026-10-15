use bevy::prelude::*;use crate::BlueRegistry;
#[derive(Default)]pub struct BlueExposurePlugin;
impl Plugin for BlueExposurePlugin{fn build(&self,app:&mut App){app.init_resource::<BlueRegistry>().register_type::<BlueRegistry>();}}
