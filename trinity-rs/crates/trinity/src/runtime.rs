use bevy::prelude::*;
use delegate::delegate;
use crate::{TrinityPlugins, TrinityVerificationPlugins};
pub struct TrinityRuntime { app: App }
impl Default for TrinityRuntime { fn default() -> Self { Self::new() } }
impl TrinityRuntime {
    #[must_use] pub fn new() -> Self { let mut app = App::new(); app.add_plugins(TrinityPlugins); Self { app } }
    #[must_use] pub fn verification() -> Self { let mut app = App::new(); app.add_plugins(TrinityVerificationPlugins); Self { app } }
    delegate! { to self.app { pub fn world(&self) -> &World; pub fn world_mut(&mut self) -> &mut World; } }
    pub fn update(&mut self) { let _ = self.app.update(); }
    #[must_use] pub fn app(&self) -> &App { &self.app }
    pub fn app_mut(&mut self) -> &mut App { &mut self.app }
}
