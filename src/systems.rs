use crate::{
    assets::WolfMap,
    components::{Spawn, WolfCamera, WolfUIFPSText},
    AssetMap, WolfAssets, WolfEntity, WolfThing, WolfTile, WolfTileBundle, WolfWorld, WolfConfig,
};

use bevy::{prelude::*, utils::petgraph::dot::Config, input::mouse::MouseMotion};

pub fn startup_system(mut commands:Commands, ass: Res<AssetServer>, mut assets: ResMut<WolfAssets>) {
    assets
        .meshes
        .insert("block", ass.load("meshes/block.gltf#Mesh0/Primitive0"));
    assets
        .meshes
        .insert("floor", ass.load("meshes/floor.gltf#Mesh0/Primitive0"));

    commands.spawn(TextBundle {
        text:Text::from_section("Hello World", TextStyle {
            font_size:24.0,
            color:Color::RED,
            ..Default::default()
        }),
        ..Default::default()
    }).insert(WolfUIFPSText);
}

fn ui_system(mut q:Query<&mut Text, With<WolfUIFPSText>>, time:Res<Time>) {
    q.single_mut().sections[0].value = format!("{:.0}", 1.0 / time.delta_seconds());
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
            if let Some(tile) = map.blocks.get(y as usize, x as usize).unwrap() {
                let tile = map.tileset.get(tile).unwrap();
                commands.spawn(WolfTileBundle::new(x, y, tile));
            }
        }
    }

    // spawn things
    for entity in map.entities.iter() {
        let mut entity = commands.spawn(WolfEntity::default());
        
       /* if entity.name == "info_player_start" {
            let mut pos = entity.pos;
            pos.z += 0.5;
            commands
                .spawn(Camera3dBundle {
                    transform: Transform::from_xyz(pos.x, pos.y, pos.z)
                        .looking_to(Vec3::new(1.0, 0.0, 0.0), Vec3::Z),
                    ..Default::default()
                })
                .insert(WolfEntity)
                .insert(WolfCamera::default());
        }*/
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

pub fn camera_system(mut cameras: Query<(&mut WolfCamera, &mut Transform)>, keys:Res<Input<KeyCode>>, time:Res<Time>, config:Res<WolfConfig>, mut mouse_motion:EventReader<MouseMotion>) {
    for (wcamera, mut transform) in cameras.iter_mut() {
        let mut v = Vec3::new(0.0, 0.0, 0.0);
        let up = Vec3::new(0.0, 0.0, 1.0);
      
        // turn
        for ev in mouse_motion.iter() {
            transform.rotate_z(-ev.delta.x  * config.turn_speed);
            let forward = transform.forward().normalize_or_zero();
            let side = forward.cross(up);
            let mut t = transform.clone();
            t.rotate_axis(side.normalize_or_zero(), -ev.delta.y * config.turn_speed);
            let side = t.forward().normalize_or_zero().cross(up);
            if side.length() > 0.1 {
                *transform = t;
            }
            let dir = t.rotation * Vec3::new(1.0, 0.0, 0.0);
            let _ = transform.looking_to(dir, Vec3::Z);
        }
        
        // movement
        if keys.pressed(config.forward_key) {
            v.y = 1.0;
        }
        if keys.pressed(config.backward_key) {
            v.y -= 1.0;
        }
        if keys.pressed(config.strife_left_key) {
            v.x -= 1.0;
        }
        if keys.pressed(config.strife_right_key) {
            v.x += 1.0;
        }
        let v = v.normalize_or_zero();
        let forward = transform.forward();
        let side = forward.cross(up).normalize_or_zero();
        let dt = time.delta_seconds();
        let speed = 10.0;
        transform.translation += forward * v.y * dt * speed;
        transform.translation += side * v.x * dt * speed;
        break;
    }
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
    app.add_systems(
        Update,
        (spawn_cam_system, tile_spawn_system, camera_system, ui_system).chain(),
    );
    app.add_systems(PostUpdate, debug_gizmos_system);
}
