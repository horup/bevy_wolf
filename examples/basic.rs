use bevy::{prelude::*};
use bevy_wolf::{
    components::{Cam, Spawn, Map},
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
    commands.spawn(Spawn::new(Map {
        map_path:"maps/basic.tmx".into(),
        ..Default::default()
    }));
    commands.spawn(Spawn::new(Cam {
        pos: (2.0, -10.0, 1.0).into(),
        yaw: 0.0,
    }));
}
