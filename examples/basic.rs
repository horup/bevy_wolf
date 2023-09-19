use bevy::{prelude::*};
use bevy_wolf::{
    components::{Cam, SpawnVariant, Spawn},
    WolfPlugin,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(WolfPlugin)
        .add_systems(Startup, startup_system)
        .run();
}

fn startup_system(mut commands: Commands) {
    commands.spawn(Spawn {
        variant: SpawnVariant::Cam { cam: Cam { pos: (1.0, -10.0, 1.0).into(), yaw: 0.0 } },
    });
}
