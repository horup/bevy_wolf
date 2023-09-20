use crate::{
    assets::WolfMap,
    components::{Spawn, WolfCamera},
    AssetMap, WolfAssets, WolfEntity, WolfTile, WolfTileBundle, WolfWorld,
};

use bevy::prelude::*;

pub fn startup_system(ass: Res<AssetServer>, mut assets: ResMut<WolfAssets>) {
    assets
        .meshes
        .insert("block", ass.load("meshes/block.gltf#Mesh0/Primitive0"));
    assets
        .meshes
        .insert("floor", ass.load("meshes/floor.gltf#Mesh0/Primitive0"));
}

pub fn spawn_cam_system(
    mut commands: Commands,
    spawns: Query<(Entity, &Spawn<WolfCamera>), Added<Spawn<WolfCamera>>>,
) {
    /* for (e, spawn) in spawns.iter() {
        let cam = spawn.variant.clone();
        let dir = Vec3::new(0.0, 1.0, 0.0);
        commands
            .entity(e)
            .insert(Camera3dBundle {
                transform: Transform::from_xyz(cam.pos.x, cam.pos.y, cam.pos.z)
                    .looking_to(dir, Vec3::Z),
                ..Default::default()
            })
            .insert(cam.clone());
    }*/
}

pub fn tile_spawn_system(
    mut commands: Commands,
    tiles: Query<(Entity, &WolfTile), Added<WolfTile>>,
    mut standard_material: ResMut<Assets<StandardMaterial>>,
    mut images: ResMut<Assets<Image>>,
    mut assets: ResMut<WolfAssets>,
    ass: Res<AssetServer>,
) {
    for (e, tile) in tiles.iter() {
        let texture = &tile.texture;
        let material = match assets.standard_materials.get(texture) {
            Some(mat) => mat.clone(),
            None => {
                let image = match assets.images.get(texture) {
                    Some(image) => image.clone(),
                    None => ass.load(format!("images/{}.png", texture)).clone(),
                };
                standard_material.add(StandardMaterial {
                    base_color_texture: Some(image),
                    unlit: true,
                    ..Default::default()
                })
            }
        };

        commands.entity(e).insert(PbrBundle {
            material: material,
            mesh: assets.meshes.get("block").unwrap(),
            transform: Transform::from_xyz(tile.pos.x as f32, tile.pos.y as f32, 0.0),
            ..Default::default()
        });
    }
}

fn load_map_system(
    mut commands: Commands,
    mut world: ResMut<WolfWorld>,
    maps: Res<Assets<WolfMap>>,
    entities: Query<Entity, With<WolfEntity>>,
) {
    let Some(handle) = &world.map_handle else {
        return;
    };

    let Some(wolf_map) = maps.get(handle) else {
        return;
    };

    // clear current wolf world of entities
    for e in entities.iter() {
        commands.entity(e).despawn_recursive();
    }
    world.map = wolf_map.clone();
    world.map_handle = None;

    let map = &world.map;
    // spawn walls
    for y in 0..map.height {
        for x in 0..map.width {
            if let Some(tile) = map.walls.get(y as usize, x as usize).unwrap() {
                let tile = map.tileset.get(tile).unwrap();
                commands.spawn(WolfTileBundle::new(x, y, tile));
            }
        }
    }

    // spawn things
    for thing in map.things.iter() {
        if thing.name == "info_player_start" {
            let mut pos = thing.pos;
            pos.z += 0.5;
            commands
                .spawn(Camera3dBundle {
                    transform: Transform::from_xyz(pos.x, pos.y, pos.z)
                        .looking_to(Vec3::new(1.0, 0.0, 0.0), Vec3::Z),
                    ..Default::default()
                })
                .insert(WolfEntity);
        }
    }

    // spawn camera
    /*commands
    .spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 0.0, 50.0)
            .looking_to(Vec3::new(0.0, 0.0, -1.0), Vec3::Z),
        ..Default::default()
    })
    .insert(WolfEntity);*/
}

pub fn debug_gizmos_system(mut gizmos: Gizmos, _time: Res<Time>) {
    // draw origin
    gizmos.ray((0.0, 0.0, 0.0).into(), (0.0, 0.0, 1.0).into(), Color::BLUE);
    gizmos.ray((0.0, 0.0, 0.0).into(), (0.0, 1.0, 0.0).into(), Color::GREEN);
    gizmos.ray((0.0, 0.0, 0.0).into(), (1.0, 0.0, 0.0).into(), Color::RED);
}

pub fn build_systems(app: &mut App) {
    app.add_systems(Startup, startup_system);
    app.add_systems(PreUpdate, (load_map_system).chain());
    app.add_systems(Update, (spawn_cam_system, tile_spawn_system).chain());
    app.add_systems(PostUpdate, debug_gizmos_system);
}
