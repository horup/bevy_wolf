use std::process::exit;

use bevy::{input::keyboard::KeyboardInput, prelude::*, render::{settings::{Backends, WgpuSettings}, texture::{ImageSampler, ImageSamplerDescriptor}, RenderPlugin}, window::{Cursor, CursorGrabMode, PresentMode}};
use bevy_wolf::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin {
            default_sampler:ImageSamplerDescriptor::nearest(),
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
        }).set(RenderPlugin {
           /* wgpu_settings:WgpuSettings {
                backends:Some(Backends::VULKAN), // not sure why needed to force to vulkan
                ..Default::default()
            },*/
            ..Default::default()
        }))
        .add_plugins(WolfPlugin)
        .add_systems(Startup, startup_system)
        .add_systems(Update, (input_system, test_system))
        .run();
}

fn test_system(mut q:Query<&mut Transform, With<WolfSprite>>, time:Res<Time>, mut config:ResMut<WolfConfig>, keys:Res<ButtonInput<KeyCode>>, world:Res<WolfWorld>){
    if keys.just_pressed(KeyCode::F9) {
        config.show_dev = !config.show_dev;
    }
}

fn input_system(keys:Res<ButtonInput<KeyCode>>, mut windows:Query<&mut Window>) {
    let window = windows.single_mut();
    if keys.just_pressed(KeyCode::Escape) {
        //window.cursor.grab_mode = CursorGrabMode::None;
        //window.cursor.visible = true;
        exit(0);
    }
}

fn startup_system(mut world:ResMut<WolfWorld>, ass:Res<AssetServer>, ) {
    world.load_map(ass.load("maps/basic.tmx"));
}
