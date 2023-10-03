use std::f32::consts::PI;

use crate::{
    assets::WolfMap,
    components::{Spawn, Timer, WolfCamera, WolfUIFPSText},
    AssetMap, DoorState, Prev, WolfPush, WolfAssets, WolfBody, WolfConfig, WolfDoor, WolfEntity,
    WolfEntityRef, WolfInstance, WolfInstanceManager, WolfInteract, WolfInteractEvent, WolfSprite,
    WolfWorld, BODY_SHAPE_BALL, BODY_SHAPE_CUBOID,
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
use parry2d::{bounding_volume::BoundingVolume, query::RayCast, na::ComplexField};

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
    mut meshes: ResMut<Assets<Mesh>>,
    world: ResMut<WolfWorld>,
    mut assets: ResMut<WolfAssets>,
) {
    let block_mesh: Handle<Mesh> = ass.load("meshes/block.gltf#Mesh0/Primitive0");
    let mut existing_materials = HashMap::new();
    for (id, material) in materials.iter() {
        existing_materials.insert(material.base_color_texture.clone(), id.clone());
    }
    for (e, we) in spawns.iter() {
        let mut transform = Transform::from_xyz(we.start_pos.x, we.start_pos.y, we.start_pos.z)
            .looking_to(Vec3::new(1.0, 0.0, 0.0), Vec3::Z);
        let mut entity = commands.entity(e);
        entity.insert(SpatialBundle {
            transform,
            ..Default::default()
        });
        entity.insert(Prev {
            component: transform,
        });

        let image = we.get_property_string("image");
        if we.has_class("camera") {
            entity
                .insert(Camera3dBundle {
                    transform,
                    ..Default::default()
                })
                .insert(WolfCamera::default());
        }

        if we.has_class("block") {
            if let Some(image) = image {
                let material = match existing_materials.get(&Some(ass.load(image))) {
                    Some(h) => materials.get_handle(*h),
                    None => {
                        let material = materials.add(StandardMaterial {
                            perceptual_roughness: 1.0,
                            metallic: 0.0,
                            base_color_texture: Some(ass.load(image)),
                            unlit: true,
                            ..Default::default()
                        });
                        existing_materials.insert(Some(ass.load(image)), material.id());
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
                    .insert(transform);
            }
        }

        if we.has_class("sprite") {
            let atlas_width = *we.get_property_int("atlas_width").unwrap_or(&1) as u8;
            let atlas_height = *we.get_property_int("atlas_height").unwrap_or(&1) as u8;
            let atlas = assets
                .sprite_meshes
                .get(atlas_height, atlas_width, &mut meshes);
            entity.insert(WolfSprite {
                atlas_height,
                atlas_width,
                ..Default::default()
            });
            let id = entity.id();
            entity
                .commands()
                .spawn(PbrBundle {
                    mesh: atlas.index(0),
                    material: materials.add(StandardMaterial {
                        alpha_mode: AlphaMode::Blend,
                        perceptual_roughness: 1.0,
                        metallic: 0.0,
                        cull_mode: None,
                        base_color_texture: image.and_then(|x| Some(ass.load(x))),
                        unlit: true,
                        ..Default::default()
                    }),
                    ..Default::default()
                })
                .insert(WolfEntityRef { entity: id });
        }

        if we.has_class("door") {
            let right = we.start_pos.as_uvec3().truncate() + UVec2::X;
            let tiles = world.map.get(right);
            for tile in tiles {
                if tile.has_class("block") {
                    transform.look_to(Vec3::Y, Vec3::Z)
                }
            }

            entity.insert(SpatialBundle {
                transform,
                ..Default::default()
            });

            entity.insert(WolfDoor {
                ..Default::default()
            });

            entity.with_children(|mut builder| {
                builder.spawn(PbrBundle {
                    mesh: assets.sprite_meshes.get(1, 1, &mut meshes).index(0),
                    material: materials.add(StandardMaterial {
                        alpha_mode: AlphaMode::Blend,
                        perceptual_roughness: 1.0,
                        metallic: 0.0,
                        cull_mode: None,
                        base_color_texture: image.and_then(|x| Some(ass.load(x))),
                        unlit: true,
                        ..Default::default()
                    }),
                    transform: Transform::from_xyz(0.0, 0.0, 0.0),
                    ..Default::default()
                });
            });
        }

        if we.has_class("body") {
            let radius = we.properties_float.get("body_radius").unwrap_or(&0.5);
            let height = we.properties_float.get("body_height").unwrap_or(&1.0);
            let shape = we.properties_int.get("body_shape").unwrap_or(&0);
            let body = WolfBody {
                height: *height,
                radius: *radius,
                shape: *shape as u8,
                ..Default::default()
            };
            entity.insert(body);
        }

        if we.has_class("interact") {
            entity.insert(WolfInteract {
                ..Default::default()
            });
        }

        if we.has_class("push") {
            for (other_e, we2) in spawns
                .iter()
                .filter(|(_, we2)| we2.start_pos == we.start_pos)
            {
                if we2.has_class("block") {
                    commands
                        .entity(other_e)
                        .insert(WolfInteract::default())
                        .insert(WolfPush::default());
                }
            }
        }
    }
}

pub fn sprite_system(
    mut commands: Commands,
    mut sprite_meshes: Query<(Entity, &mut Handle<Mesh>, &WolfEntityRef)>,
    mut transforms: Query<&mut Transform>,
    sprites: Query<(Entity, &WolfSprite)>,
    cameras: Query<Entity, With<Camera3d>>,
    mut assets: ResMut<WolfAssets>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    for camera in cameras.iter() {
        let camera_transform = transforms.get(camera).unwrap().clone();
        for (e, mut mesh_handle, r) in sprite_meshes.iter_mut() {
            match sprites.get(r.entity) {
                Ok((sprite_entity, sprite)) => {
                    let sprite_transform = *transforms
                        .get(sprite_entity)
                        .unwrap_or(&Transform::default());
                    if let Ok(mut transform) = transforms.get_mut(e) {
                        transform.translation = sprite_transform.translation;
                        let atlas = assets.sprite_meshes.get(
                            sprite.atlas_height,
                            sprite.atlas_width,
                            &mut meshes,
                        );

                        let handle = atlas.index(sprite.index as u16);
                        *mesh_handle = handle;
                        let z = transform.translation.z;
                        transform
                            .look_at(camera_transform.translation.truncate().extend(z), Vec3::Z);
                    }
                }
                Err(_) => commands.entity(e).despawn(),
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
}

pub fn camera_system(
    mut cameras: Query<(&mut Transform, &mut WolfEntity), With<WolfCamera>>,
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
    config: Res<WolfConfig>,
    mut mouse_motion: EventReader<MouseMotion>,
) {
    for (mut transform, mut we) in cameras.iter_mut() {
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
        we.start_pos = transform.translation;
        break;
    }
}

fn interactor_system(
    mut cameras: Query<(Entity, &mut Transform, &mut WolfEntity), With<WolfCamera>>,
    interacts: Query<(&Transform, &WolfBody, &WolfInteract), Without<WolfCamera>>,
    keys: Res<Input<KeyCode>>,
    config: Res<WolfConfig>,
    world: Res<WolfWorld>,
    mut writer: EventWriter<WolfInteractEvent>,
) {
    for (camera_entity, transform, we) in cameras.iter_mut() {
        if keys.just_pressed(config.interaction_key) {
            let v = transform.forward(); //transform.rotation * Vec3::new(1.0, 0.0, 0.0);
            let ray = parry2d::query::Ray::new(
                [transform.translation.x, transform.translation.y].into(),
                [v.x, v.y].into(),
            );
            let l = 1.0;
            for (e, p) in world
                .grid
                .query_around(transform.translation.truncate(), 2.0)
            {
                let Ok((_, wb, _wi)) = interacts.get(e) else {
                    continue;
                };
                let s = parry2d::shape::Cuboid::new([wb.radius, wb.radius].into());
                if let Some(_) = s.cast_ray_and_get_normal(&[p.x, p.y].into(), &ray, l, true) {
                    writer.send(WolfInteractEvent {
                        interactor: camera_entity,
                        entity: e,
                    });
                }
            }
        }

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
                count: 0,
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
    mut instances: Query<(&mut WolfInstance<StandardMaterial>, &mut Transform)>,
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
        for (mut instance, t) in instances.iter_mut() {
            if instance_manager.instance == *instance {
                count += 1;
                if t.is_changed() {
                    instance.request_redraw = true;
                }
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

        if count != instance_manager.count {
            instance_manager.request_redraw = true;
            instance_manager.count = count;
        }

        if instance_manager.request_redraw {
            let mut instance_mesh = meshes.get(&instance_manager.instance.mesh).unwrap().clone();
            instance_mesh.duplicate_vertices();
            let vertex_count = instance_mesh.count_vertices() as u32;
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

            let l = count as usize * vertex_count as usize;
            let mut new_positions: Vec<[f32; 3]> = Vec::with_capacity(l);
            let mut new_uvs: Vec<[f32; 2]> = Vec::with_capacity(l);
            let mut new_normals: Vec<[f32; 3]> = Vec::with_capacity(l);
            let mut indicies: Vec<u32> = Vec::with_capacity(l);

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
                    for p in normals {
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

pub fn debug_gizmos_system(
    mut gizmos: Gizmos,
    _time: Res<Time>,
    sprites: Query<&Transform, With<WolfSprite>>,
    config: Res<WolfConfig>,
) {
    // draw origin
    gizmos.ray((0.0, 0.0, 0.0).into(), (0.0, 0.0, 1.0).into(), Color::BLUE);
    gizmos.ray((0.0, 0.0, 0.0).into(), (0.0, 1.0, 0.0).into(), Color::GREEN);
    gizmos.ray((0.0, 0.0, 0.0).into(), (1.0, 0.0, 0.0).into(), Color::RED);

    if config.show_dev {
        for s in sprites.iter() {
            let r = s.rotation * Vec3::new(1.0, 0.0, 0.0);

            gizmos.ray(s.translation, r, Color::RED);
        }
    }
}

pub fn spatial_hash_system(
    mut world: ResMut<WolfWorld>,
    entities: Query<(Entity, &WolfEntity, &Transform)>,
    time: Res<Time>,
) {
    world.grid.clear();
    for (e, we, t) in entities.iter() {
        world.grid.insert_or_replace(e, t.translation.truncate());
    }
}

pub fn prev_system(mut transforms: Query<(&Transform, &mut Prev<Transform>)>) {
    for (current, mut prev) in transforms.iter_mut() {
        prev.component = *current;
    }
}

pub fn body_system(
    bodies: Query<(Entity, &WolfBody)>,
    mut transforms: Query<&mut Transform>,
    mut prev_transforms: Query<&Prev<Transform>>,
    mut world: ResMut<WolfWorld>,
) {
    let mut contacts = Vec::with_capacity(8);
    for (entity, body) in bodies.iter() {
        let Ok(transform) = transforms.get(entity) else {
            continue;
        };
        let Ok(prev_transform) = prev_transforms.get(entity) else {
            continue;
        };
        let v = transform.translation - prev_transform.translation;
        let mut vl = v.length();
        if vl == 0.0 {
            continue;
        }

        let d = v.normalize_or_zero().truncate();
        let max_step = 0.1;
        let prev_translate = prev_transform.component.translation.clone();
        let mut new_translation = prev_translate.truncate();

        while vl > 0.0 {
            let step = vl.min(max_step);
            vl -= step;
            let mut v = d * step;
            let s = new_translation;
            let mut e = s + v;

            let q_radius = 4.0;
            contacts.clear();
            let mut retry = true;
            let mut try_count = 0;
            while retry && try_count < 5 {
                try_count += 1;
                retry = false;
                for (other_e, other_pos) in world.grid.query_around(e, q_radius) {
                    if body.disabled {
                        continue;
                    }
                    if entity == other_e {
                        continue;
                    }
                    let Ok((_, other_body)) = bodies.get(other_e) else {
                        continue;
                    };
                    if other_body.disabled {
                        continue;
                    }
                    let a_cuboid = parry2d::shape::Cuboid::new([body.radius, body.radius].into());
                    let a_ball = parry2d::shape::Ball::new(body.radius);
                    let b_cuboid =
                        parry2d::shape::Cuboid::new([other_body.radius, other_body.radius].into());
                    let b_ball = parry2d::shape::Ball::new(other_body.radius);
                    if let Ok(Some(c)) = parry2d::query::contact(
                        &[e.x, e.y].into(),
                        match body.shape {
                            BODY_SHAPE_CUBOID => &a_cuboid as &dyn parry2d::shape::Shape,
                            BODY_SHAPE_BALL => &a_ball as &dyn parry2d::shape::Shape,
                            _ => &a_cuboid as &dyn parry2d::shape::Shape,
                        },
                        &[other_pos.x, other_pos.y].into(),
                        match other_body.shape {
                            BODY_SHAPE_CUBOID => &b_cuboid as &dyn parry2d::shape::Shape,
                            BODY_SHAPE_BALL => &b_ball as &dyn parry2d::shape::Shape,
                            _ => &a_cuboid as &dyn parry2d::shape::Shape,
                        },
                        0.0,
                    ) {
                        contacts.push(c);
                    }
                }

                if contacts.len() > 0 {
                    let mut contact = contacts[0];
                    let mut dist = contact.dist;
                    for c in contacts.drain(..) {
                        if c.dist < dist {
                            dist = c.dist;
                            contact = c;
                        }
                    }
                    let n = Vec2::new(contact.normal1[0], contact.normal1[1]);
                    let push_back = n * dist * 1.2;
                    v += push_back;
                    retry = true;
                }

                new_translation += v;
                e = new_translation;
                v = Vec2::default();
            }
        }

        let Ok(mut transform) = transforms.get_mut(entity) else {
            continue;
        };

        transform.translation = new_translation.extend(prev_translate.z);
        let new_translation = new_translation.extend(prev_translate.z);

        world
            .grid
            .insert_or_replace(entity, new_translation.truncate());
    }
}

fn door_system(
    mut interact_events: EventReader<WolfInteractEvent>,
    mut doors: Query<(Entity, &mut WolfDoor, &Children)>,
    time: Res<Time>,
    mut transforms: Query<(&mut Transform)>,
    mut bodies: Query<&mut WolfBody>,
    world: Res<WolfWorld>,
) {
    let dt_secs = time.delta_seconds();
    let door_timer = 0.5;
    let auto_close_timer = 3.0;
    for ev in interact_events.iter() {
        let Ok((_, mut door, _)) = doors.get_mut(ev.entity) else {
            continue;
        };
        match &mut door.state {
            DoorState::Closed => {
                door.state = DoorState::Opening {
                    opening: Timer::start(door_timer),
                };
            }
            DoorState::Closing { closing: _ } => {}
            DoorState::Opening { opening: _ } => {}
            DoorState::Open {
                auto_close_timer: _,
            } => {}
        }
    }

    for (e, mut door, children) in doors.iter_mut() {
        let Ok(t) = transforms.get(e) else {
            continue;
        };
        let Ok(mut door_body) = bodies.get_mut(e) else {
            continue;
        };
        let p = t.translation.clone();
        match &mut door.state {
            crate::DoorState::Closed => {
                door_body.disabled = false;
            }
            crate::DoorState::Closing { closing } => {
                door_body.disabled = false;
                closing.tick(dt_secs);
                if closing.is_done() {
                    door.state = DoorState::Closed;
                }
            }
            crate::DoorState::Opening { opening } => {
                door_body.disabled = false;
                opening.tick(dt_secs);
                if opening.is_done() {
                    door.state = DoorState::Open {
                        auto_close_timer: Timer::start(auto_close_timer),
                    };
                }
            }
            crate::DoorState::Open { auto_close_timer } => {
                door_body.disabled = true;
                let mut blocked = false;
                for (other_e, other_p) in world.grid.query(p.truncate(), 1.0) {
                    if other_e != e {
                        if let Ok(_) = bodies.get(other_e) {
                            let a = 0.49;
                            let ab1 = parry2d::bounding_volume::Aabb::from_half_extents(
                                [p.x, p.y].into(),
                                [a, a].into(),
                            );
                            let ab2 = parry2d::bounding_volume::Aabb::from_half_extents(
                                [other_p.x, other_p.y].into(),
                                [a, a].into(),
                            );
                            if ab1.intersects(&ab2) {
                                blocked = true;
                                break;
                            }
                        }
                    }
                }
                if !blocked {
                    auto_close_timer.tick(dt_secs);
                    if auto_close_timer.is_done() {
                        door.state = DoorState::Closing {
                            closing: Timer::start(door_timer),
                        };
                    }
                }
            }
        }
        for e in children.iter() {
            if let Ok(mut transform) = transforms.get_mut(*e) {
                match &door.state {
                    crate::DoorState::Closed => {
                        transform.translation.x = 0.0;
                    }
                    crate::DoorState::Closing { closing } => {
                        transform.translation.x = 1.0 - closing.alpha();
                    }
                    crate::DoorState::Opening { opening } => {
                        transform.translation.x = opening.alpha()
                    }
                    crate::DoorState::Open {
                        auto_close_timer: _,
                    } => {
                        transform.translation.x = 1.0;
                    }
                }
            }
        }
    }
}

pub fn push_system(
    mut interact_events: EventReader<WolfInteractEvent>,
    mut pushes: Query<(Entity, &mut WolfPush)>,
    mut transforms:Query<&mut Transform>,
    time: Res<Time>
) {
    for ev in interact_events.iter() {
        let interactor = ev.interactor;
        let Ok(interactor_transform) = transforms.get(interactor) else {continue;};
        let Ok(push_transform) = transforms.get(ev.entity) else {continue;};
        let e = ev.entity;
        let Ok((_, mut push)) = pushes.get_mut(e) else { continue; };

        let p1 = push_transform.translation.truncate().as_ivec2();
        let p2 = interactor_transform.translation.truncate().as_ivec2();
        let v = p1 - p2;
        if v.x == 0 || v.y == 0 {
            let v = v.as_vec2();
            if v.length() > 0.0 {
                push.vel = v.extend(0.0);
            }
        }
    }

    for (e, push) in pushes.iter() {
        if push.vel.length() > 0.0 {
            let Ok(mut t) = transforms.get_mut( e) else { continue;};
            t.translation += push.vel * time.delta_seconds();
        }
    }
}

pub fn post_push_system(pushes: Query<(Entity, &mut WolfPush, &Prev<Transform>, &Transform)>, mut commands: Commands) {
    for (e, push, prev_transform, transform) in pushes.iter() {
        if push.vel.length() > 0.0 {
            let change = transform.translation - prev_transform.translation;
            if change.length() <= 0.0 {
                commands.entity(e).remove::<WolfPush>();
            }
        }
    }
}

pub fn build_systems(app: &mut App) {
    app.add_systems(Startup, startup_system);
    app.add_systems(PreUpdate, (load_map_system).chain());
    app.add_systems(
        Update,
        (
            spawn_system,
            prev_system,
            spatial_hash_system,
            camera_system,
            interactor_system,
            push_system,
            body_system,
            post_push_system,
            door_system,
            sprite_system,
            ui_system,
            spatial_hash_system,
            instance_manager_spawn_system,
            instance_manage_render_system,
        )
            .chain(),
    );
    app.add_systems(PostUpdate, debug_gizmos_system);
}
