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

fn test_system(mut commands:Commands, mut q:Query<(Entity, &WolfEntity, &mut WolfInstance<StandardMaterial>, &mut Transform)>){
    for (e, we, mut wi, mut t) in q.iter_mut() {
        //t.translation.x += 0.001;
        //wi.request_redraw = true;
        commands.entity(e).despawn();
        break;
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
