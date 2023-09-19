pub mod components;
pub mod systems;

use bevy::prelude::*;
pub struct WolfPlugin;

impl Plugin for WolfPlugin {
    fn build(&self, app: &mut App) {
        systems::build(app);
    }
}
