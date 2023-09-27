use bevy::prelude::*;

mod components;
pub use components::*;

mod resources;
pub use resources::*;

mod systems;

mod assets;
pub use assets::*;

pub struct WolfPlugin;

impl Plugin for WolfPlugin {
    fn build(&self, app: &mut App) {
        systems::build_systems(app);
        assets::build_assets(app);
        resources::build_resources(app);
    }
}

