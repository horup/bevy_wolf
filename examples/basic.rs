use bevy::prelude::*;
use bevy_wolf::WolfPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(WolfPlugin)
        .add_systems(Startup, startup_system)
        .run();
}

fn startup_system() {
    
}
