use std::f32::consts::PI;

use crate::{
    assets::WolfMap,
    components::{Spawn, WolfCamera, WolfUIFPSText},
    AssetMap, WolfAssets, WolfConfig, WolfEntity, WolfInstance, WolfInstanceManager, WolfSprite,
    WolfThing, WolfTile, WolfWorld,
};

use bevy::{
    input::mouse::MouseMotion,
    prelude::*,
    render::{
        mesh::{Indices, VertexAttributeValues},
        primitives::Aabb,
    },
    utils::{petgraph::dot::Config, HashMap},
};

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

fn ui_system(
    mut q: Query<&mut Text, With<WolfUIFPSText>>,
    time: Res<Time>,
    mut wolf_world: ResMut<WolfWorld>,
) {
    wolf_world.updates += 1;
    if wolf_world.updates % 100 == 0 {
        q.single_mut().sections[0].value = format!("{:.0}", 1.0 / time.delta_seconds());
    }
}

pub fn spawn_system(
    mut commands: Commands,
    spawns: Query<(Entity, &WolfEntity), Added<WolfEntity>>,
    ass: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let block_mesh: Handle<Mesh> = ass.load("meshes/block.gltf#Mesh0/Primitive0");
    let sprite_mesh: Handle<Mesh> = ass.load("meshes/sprite.gltf#Mesh0/Primitive0");
    //let existing_materials = materials.iter().map(|x|x.clone()).collect();
    let mut existing_materials = HashMap::new();
    for (id, material) in materials.iter() {
        existing_materials.insert(material.base_color_texture.clone(), id.clone());
    }
    for (e, we) in spawns.iter() {
        let mut entity = commands.entity(e);
        if we.has_class("camera") {
            entity
                .insert(Camera3dBundle {
                    transform: Transform::from_xyz(we.pos.x, we.pos.y, 0.5)
                        .looking_to(Vec3::new(1.0, 0.0, 0.0), Vec3::Z),
                    ..Default::default()
                })
                .insert(WolfCamera::default());
        }

        if we.has_class("block") {
            /*entity.insert(PbrBundle {
                mesh:block_mesh.clone(),
                material:materials.add(StandardMaterial {
                    perceptual_roughness:1.0,
                    metallic:0.0,
                    base_color_texture:Some(ass.load(&we.image)),
                    unlit:true,
                    ..Default::default()
                }),
                transform:Transform::from_xyz(we.index.x as f32, we.index.y as f32, 0.0).looking_to(Vec3::new(0.0, 1.0, 0.0), Vec3::Z),
                ..Default::default()
            });*/
            let material = match existing_materials.get(&Some(ass.load(&we.image))) {
                Some(h) => materials.get_handle(*h),
                None => {
                    let material = materials.add(StandardMaterial {
                        perceptual_roughness: 1.0,
                        metallic: 0.0,
                        base_color_texture: Some(ass.load(&we.image)),
                        unlit: true,
                        ..Default::default()
                    });
                    existing_materials.insert(Some(ass.load(&we.image)), material.id());
                    material
                }
            };

            entity
                .insert(WolfInstance {
                    mesh: block_mesh.clone(),
                    material,
                    request_redraw: true,
                    ..Default::default()
                })
                .insert(
                    Transform::from_xyz(we.index.x as f32, we.index.y as f32, 0.0)
                        .looking_to(Vec3::new(0.0, 1.0, 0.0), Vec3::Z),
                );
        }

        if we.has_class("sprite") {
            entity
                .insert(PbrBundle {
                    mesh: sprite_mesh.clone(),
                    material: materials.add(StandardMaterial {
                        alpha_mode: AlphaMode::Blend,
                        perceptual_roughness: 1.0,
                        metallic: 0.0,
                        base_color_texture: Some(ass.load(&we.image)),
                        unlit: true,
                        ..Default::default()
                    }),
                    transform: Transform::from_xyz(we.pos.x, we.pos.y, 0.0)
                        .looking_to(Vec3::new(0.0, 1.0, 0.0), Vec3::Z),
                    ..Default::default()
                })
                .insert(WolfSprite {});
        }
    }
}

pub fn sprite_system(
    sprites: Query<Entity, With<WolfSprite>>,
    mut transforms: Query<&mut Transform>,
    cameras: Query<Entity, With<Camera3d>>,
) {
    for camera in cameras.iter() {
        let camera_transform = transforms.get(camera).unwrap().clone();
        for sprite in sprites.iter() {
            if let Ok(mut transform) = transforms.get_mut(sprite) {
                //let t = transform.translation - camera_transform.translation;
                let z = transform.translation.z;
                transform.look_at(camera_transform.translation.truncate().extend(z), Vec3::Z);
                transform.rotate_z(PI / 2.0);
            }
        }
    }
}

fn load_map_system(
    mut commands: Commands,
    mut world: ResMut<WolfWorld>,
    maps: Res<Assets<WolfMap>>,
    entities: Query<Entity, With<WolfEntity>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
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
    for layer in map.layers.iter() {
        for y in 0..map.height {
            for x in 0..map.width {
                if let Some(wolf_entity) = layer.get(y as usize, x as usize).unwrap() {
                    commands.spawn(wolf_entity.clone());
                }
            }
        }
    }
    let size = wolf_map.height.max(wolf_map.width) as f32;
    // spawn floor
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane::from_size(size))),
        transform: Transform::from_xyz(size / 2.0, size / 2.0, 0.0).looking_to(Vec3::Y, Vec3::Z),
        material: materials.add(StandardMaterial {
            base_color: Color::rgb_u8(120, 120, 120),
            metallic: 0.0,
            perceptual_roughness: 1.0,
            unlit: true,
            ..Default::default()
        }),
        ..Default::default()
    });

    // spawn cealing
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane::from_size(64.0))),
        transform: Transform::from_xyz(size / 2.0, size / 2.0, 1.0).looking_to(Vec3::Y, -Vec3::Z),
        material: materials.add(StandardMaterial {
            base_color: Color::rgb_u8(56, 56, 56),
            metallic: 0.0,
            perceptual_roughness: 1.0,
            unlit: true,
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

pub fn instance_manager_spawn_system(
    mut commands: Commands,
    instance_managers: Query<&WolfInstanceManager<StandardMaterial>>,
    instances: Query<(&WolfInstance<StandardMaterial>, &Transform)>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let mut existing = HashMap::new();
    for wi in instance_managers.iter() {
        existing.insert(&wi.instance, ());
    }
    let mut new = HashMap::new();
    for (wi, t) in instances.iter() {
        if existing.contains_key(wi) == false {
            new.insert(wi, ());
        }
    }

    for instance in new.keys() {
        let ins = (*instance).clone();
        let instance_mesh = meshes.get(&instance.mesh).unwrap().clone();
        let mesh = meshes.add(instance_mesh);
        commands
            .spawn(WolfInstanceManager {
                instance: ins.clone(),
                request_redraw: true,
            })
            .insert(PbrBundle {
                mesh,
                material: ins.material.clone(),
                ..Default::default()
            });
    }

    // todo cleanup
}

pub fn instance_manage_render_system(
    mut commands: Commands,
    mut instances: Query<(&mut WolfInstance<StandardMaterial>, &Transform)>,
    mut instance_managers: Query<(
        Entity,
        &mut WolfInstanceManager<StandardMaterial>,
        &Handle<Mesh>,
        &mut Aabb,
    )>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    for (entity, mut instance_manager, mesh, mut aabb) in instance_managers.iter_mut() {
        let mut count = 0;
        for (mut instance, _) in instances.iter_mut() {
            if instance_manager.instance == *instance {
                count += 1;
                if instance.request_redraw {
                    instance_manager.request_redraw = true;
                    instance.request_redraw = false;
                }
            }
        }

        if count == 0 {
            commands.entity(entity).despawn_recursive();
            continue;
        }

        if instance_manager.request_redraw {
            let mut instance_mesh = meshes.get(&instance_manager.instance.mesh).unwrap().clone();
            instance_mesh.duplicate_vertices();
            let vertex_count = instance_mesh.count_vertices();
            let VertexAttributeValues::Float32x3(positions) =
                instance_mesh.attribute(Mesh::ATTRIBUTE_POSITION).unwrap()
            else {
                panic!()
            };
            let VertexAttributeValues::Float32x2(uvs) =
                instance_mesh.attribute(Mesh::ATTRIBUTE_UV_0).unwrap()
            else {
                panic!()
            };
            let VertexAttributeValues::Float32x3(normals) =
                instance_mesh.attribute(Mesh::ATTRIBUTE_NORMAL).unwrap()
            else {
                panic!()
            };

            let mut new_positions: Vec<[f32; 3]> = Vec::with_capacity(count * vertex_count);
            let mut new_uvs: Vec<[f32; 2]> = Vec::with_capacity(count * vertex_count);
            let mut new_normals: Vec<[f32; 3]> = Vec::with_capacity(count * vertex_count);
            let mut indicies: Vec<u32> = Vec::with_capacity(count * vertex_count);

            for (instance, transform) in instances.iter() {
                if instance_manager.instance == *instance {
                    for p in positions {
                        let p: Vec3 = p.clone().into();
                        let p = transform.transform_point(p);
                        new_positions.push(p.into());
                        indicies.push(indicies.len() as u32);
                    }
                    for uv in uvs {
                        new_uvs.push(uv.clone());
                    }
                    for p in positions {
                        new_normals.push(p.clone());
                    }
                }
            }

            let mesh = meshes.get_mut(mesh).unwrap();
            mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, new_positions);
            mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, new_uvs);
            mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, new_normals);
            mesh.set_indices(Some(Indices::U32(indicies)));

            if let Some(new_aabb) = mesh.compute_aabb() {
                *aabb = new_aabb;
            }

            for (instance, transform) in instances.iter() {
                if instance_manager.instance == *instance {}
            }

            instance_manager.request_redraw = false;
        }
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
            spawn_system,
            camera_system,
            sprite_system,
            ui_system,
            instance_manager_spawn_system,
            instance_manage_render_system,
        )
            .chain(),
    );
    app.add_systems(PostUpdate, debug_gizmos_system);
}
