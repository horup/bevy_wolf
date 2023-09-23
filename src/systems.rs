use std::f32::consts::PI;

use crate::{
    assets::WolfMap,
    components::{Spawn, WolfCamera, WolfUIFPSText},
    AssetMap, WolfAssets, WolfConfig, WolfEntity, WolfSprite, WolfThing, WolfTile, WolfTileBundle,
    WolfWorld,
};

use bevy::{input::mouse::MouseMotion, prelude::*, utils::petgraph::dot::Config};

pub fn startup_system(
    mut commands: Commands,
    ass: Res<AssetServer>,
    mut assets: ResMut<WolfAssets>,
) {
    assets
        .meshes
        .insert("block", ass.load("meshes/block.gltf#Mesh0/Primitive0"));
    assets
        .meshes
        .insert("floor", ass.load("meshes/floor.gltf#Mesh0/Primitive0"));
    assets
        .meshes
        .insert("sprite", ass.load("meshes/sprite.gltf#Mesh0/Primitive0"));

    commands
        .spawn(TextBundle {
            text: Text::from_section(
                "Hello World",
                TextStyle {
                    font_size: 24.0,
                    color: Color::RED,
                    ..Default::default()
                },
            ),
            ..Default::default()
        })
        .insert(WolfUIFPSText);
}

fn ui_system(mut q: Query<&mut Text, With<WolfUIFPSText>>, time: Res<Time>) {
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

pub fn sprite_spawn_system(
    mut commands: Commands,
    sprites: Query<(Entity, &WolfSprite), Added<WolfSprite>>,
    mut standard_material: ResMut<Assets<StandardMaterial>>,
    mut assets: ResMut<WolfAssets>,
    ass: Res<AssetServer>,
) {
    for (e, sp) in sprites.iter() {
        let texture = &sp.texture;
        let material = match assets.standard_materials.get(texture) {
            Some(mat) => mat.clone(),
            None => {
                let image = match assets.images.get(texture) {
                    Some(image) => image.clone(),
                    None => ass.load(format!("images/{}.png", texture)).clone(),
                };
                standard_material.add(StandardMaterial {
                    base_color_texture: Some(image),
                    alpha_mode:AlphaMode::Blend,
                    metallic:0.0,
                    perceptual_roughness:1.0,
                    unlit: false,
                    ..Default::default()
                })
            }
        };

        commands
            .entity(e)
            .insert(assets.meshes.get("sprite").unwrap())
            .insert(material)
            .insert(Visibility::default())
            .insert(ComputedVisibility::default()).insert(GlobalTransform::default());
    }
}

pub fn sprite_system(sprites:Query<Entity, With<WolfSprite>>, mut transforms:Query<&mut Transform>, cameras:Query<Entity, With<Camera3d>>) {
    for camera in cameras.iter() {
        let camera_transform = transforms.get(camera).unwrap().clone();
        for sprite in sprites.iter() {
            if let Ok(mut transform) = transforms.get_mut(sprite) {
                //let t = transform.translation - camera_transform.translation;
                let z = transform.translation.z;
                transform.look_at(camera_transform.translation.truncate().extend(z), Vec3::Z);
                transform.rotate_z(PI/2.0);
            }
        }
    }
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
       /*  let material = match assets.standard_materials.get(texture) {
            Some(mat) => mat.clone(),
            None => {
                let image = match assets.images.get(texture) {
                    Some(image) => image.clone(),
                    None => ass.load(format!("images/{}.png", texture)).clone(),
                };
                standard_material.add(StandardMaterial {
                    base_color_texture: Some(image),
                    metallic:0.0,
                    perceptual_roughness:1.0,
                    unlit: false,
                    ..Default::default()
                })
            }
        };*/

        let material = standard_material.add(StandardMaterial {
            base_color_texture:Some(ass.load(format!("images/{}.png", texture))),
            metallic:0.0,
            perceptual_roughness:1.0,
            ..Default::default()
        });

        commands.entity(e).insert(PbrBundle {
            material: material,
            mesh: assets.meshes.get("block").unwrap(),
            transform: Transform::from_xyz(tile.pos.x as f32, tile.pos.y as f32, 0.0).looking_to(Vec3::new(0.0, 1.0, 0.0), Vec3::Z),
            ..Default::default()
        });
    }
}

fn load_map_system(
    mut commands: Commands,
    mut world: ResMut<WolfWorld>,
    maps: Res<Assets<WolfMap>>,
    entities: Query<Entity, With<WolfEntity>>,
    mut meshes:ResMut<Assets<Mesh>>,
    mut materials:ResMut<Assets<StandardMaterial>>
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

    // spawn entities
    for e in &map.entities {
        let x = e.pos.x;
        let y = e.pos.y;
        let mut pos = e.pos;
        let mut entity = commands.spawn(WolfEntity::default());
        if e.has_class("camera") {
            pos.z = 0.5;
            entity
                .insert(Camera3dBundle {
                    transform: Transform::from_xyz(pos.x, pos.y, pos.z)
                        .looking_to(Vec3::new(1.0, 0.0, 0.0), Vec3::Z),
                    ..Default::default()
                })
                .insert(WolfCamera::default());
        }

        if e.has_class("sprite") {
            entity
                .insert(WolfSprite {
                    texture: e.name.clone(),
                })
                .insert(Transform::from_xyz(pos.x, pos.y, pos.z));
        }

        if e.has_class("light") {
            /*entity.insert(ComputedVisibility::default());
            entity.with_children(|b|{
                dbg!("ha");
                b.spawn(PointLightBundle {
                    point_light:PointLight {
                        shadows_enabled:true,
                        intensity:1600.0,
                        color:Color::WHITE,
                        ..Default::default()
                    },
                    ..Default::default()
                });
            });*/

            commands.spawn(PointLightBundle {
                transform:Transform::from_xyz(x, y, 0.5),
                point_light:PointLight {
                    intensity:100.0,
                    color:Color::WHITE,
                    ..Default::default()
                },
                ..Default::default()
            });
            
        }
    }

    // spawn floor
    commands.spawn(PbrBundle {
        mesh:meshes.add(Mesh::from(shape::Plane::from_size(64.0))),
        transform:Transform::default().looking_to(Vec3::Y, Vec3::Z),
        material:materials.add(StandardMaterial {
            base_color:Color::rgb_u8(120, 120, 120),
            metallic:0.0,
            perceptual_roughness:1.0,
            ..Default::default()
        }),
        ..Default::default()
    });

    // spawn cealing
    commands.spawn(PbrBundle {
        mesh:meshes.add(Mesh::from(shape::Plane::from_size(64.0))),
        transform:Transform::from_xyz(0.0, 0.0, 1.0).looking_to(Vec3::Y, -Vec3::Z),
        material:materials.add(StandardMaterial {
            base_color:Color::rgb_u8(56, 56, 56),
            metallic:0.0,
            perceptual_roughness:1.0,
            ..Default::default()
        }),
        ..Default::default()
    });

    // spawn camera
    /*commands
    .spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 0.0, 50.0)
            .looking_to(Vec3::new(0.0, 0.0, -1.0), Vec3::Z),
        ..Default::default()
    })
    .insert(WolfEntity);*/
}

pub fn camera_system(
    mut cameras: Query<(&mut WolfCamera, &mut Transform)>,
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
    config: Res<WolfConfig>,
    mut mouse_motion: EventReader<MouseMotion>,
) {
    for (wcamera, mut transform) in cameras.iter_mut() {
        let mut v = Vec3::new(0.0, 0.0, 0.0);
        let up = Vec3::new(0.0, 0.0, 1.0);

        // turn
        for ev in mouse_motion.iter() {
            transform.rotate_z(-ev.delta.x * config.turn_speed);
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
        (
            spawn_cam_system,
            tile_spawn_system,
            sprite_spawn_system,
            camera_system,
            sprite_system,
            ui_system,
        )
            .chain(),
    );
    app.add_systems(PostUpdate, debug_gizmos_system);
}
