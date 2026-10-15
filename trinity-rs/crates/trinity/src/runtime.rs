use bevy::prelude::*;
use delegate::delegate;

use crate::{TrinityPlugins, TrinityVerificationPlugins};

/// Opinionated Carbon Trinity runtime. Bevy is exposed directly: there is no
/// second scheduler, ECS, asset graph, reflection model, or render graph hidden behind Trinity.
pub struct TrinityRuntime {
    app: App,
}

impl Default for TrinityRuntime {
    fn default() -> Self {
        Self::new()
    }
}

impl TrinityRuntime {
    /// Production-oriented runtime using the Bevy 3D/render plugin stack selected by this workspace.
    #[must_use]
    pub fn new() -> Self {
        let mut app = App::new();
        app.add_plugins(TrinityPlugins);
        Self { app }
    }

    /// Deterministic runtime for tests, Kani harnesses, and tools that must not initialize a GPU.
    #[must_use]
    pub fn verification() -> Self {
        let mut app = App::new();
        app.add_plugins(TrinityVerificationPlugins);
        Self { app }
    }

    delegate! {
        to self.app {
            pub fn world(&self) -> &World;
            pub fn world_mut(&mut self) -> &mut World;
        }
    }

    pub fn update(&mut self) {
        let _ = self.app.update();
    }

    #[must_use]
    pub fn app(&self) -> &App {
        &self.app
    }
    pub fn app_mut(&mut self) -> &mut App {
        &mut self.app
    }
}
