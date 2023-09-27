use std::process::exit;

use bevy::{prelude::*, render::texture::ImageSampler, window::{PresentMode, Cursor, CursorGrabMode}};
use bevy_wolf::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin {
            default_sampler:ImageSampler::nearest_descriptor(),
            ..Default::default()
        }).set(WindowPlugin {
            primary_window:Some(Window {
                present_mode:PresentMode::Immediate,
                cursor:Cursor {
                    visible:false,
                    grab_mode:CursorGrabMode::Locked,
                    ..Default::default()
                },
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_plugins(WolfPlugin)
        .add_systems(Startup, startup_system)
        .add_systems(Update, (input_system, test_system))
        .run();
}

fn test_system(mut q:Query<&mut WolfEntity, With<WolfSprite>>, time:Res<Time>, mut config:ResMut<WolfConfig>, keys:Res<Input<KeyCode>>, world:Res<WolfWorld>){
    if keys.just_pressed(KeyCode::F9) {
        config.show_dev = !config.show_dev;
    }
    let mut hits = 0;
    for mut s in q.iter_mut() {
        //s.index += time.delta_seconds();
        //s.pos.x += time.delta_seconds();
        //s.facing += time.delta_seconds();
        let iter = world.grid.query_around(s.origin, 0.1);
        hits += iter.count();
    }
}

fn input_system(keys:Res<Input<KeyCode>>, mut windows:Query<&mut Window>) {
    let mut window = windows.single_mut();
    if keys.just_pressed(KeyCode::Escape) {
        //window.cursor.grab_mode = CursorGrabMode::None;
        //window.cursor.visible = true;
        exit(0);
    }
}

fn startup_system(mut world:ResMut<WolfWorld>, ass:Res<AssetServer>, ) {
    world.load_map(ass.load("maps/basic.tmx"));
}
