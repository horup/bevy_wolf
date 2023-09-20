use bevy::{prelude::*, render::texture::ImageSampler};
use bevy_wolf::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin {
            default_sampler:ImageSampler::nearest_descriptor(),
            ..Default::default()
        }))
        .add_plugins(WolfPlugin)
        .add_systems(Startup, startup_system)
        .run();
}

fn startup_system(mut world:ResMut<WolfWorld>, ass:Res<AssetServer>, ) {
    world.load_map(ass.load("maps/basic.tmx"));
}
