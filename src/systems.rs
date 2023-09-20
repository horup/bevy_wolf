use crate::{
    assets::WolfMap,
    components::{WolfCamera, Spawn},
    WolfWorld, WolfEntity, WolfTileBundle,
};

use bevy::prelude::*;

pub fn startup_system(
    _commands: Commands,
    _meshes: ResMut<Assets<Mesh>>,
    _materials: ResMut<Assets<StandardMaterial>>,
) {
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

fn load_map_system(mut commands:Commands, mut wolf_world: ResMut<WolfWorld>, wolf_maps: Res<Assets<WolfMap>>, entities:Query<Entity, With<WolfEntity>>) {
    let Some(handle) = &wolf_world.map_handle else {
        return;
    };

    let Some(wolf_map) = wolf_maps.get(handle) else {
        return;
    };

    // clear current world of entities
    for e in entities.iter() {
        commands.entity(e).despawn_recursive();
    }
    wolf_world.map = wolf_map.clone();
    wolf_world.map_handle = None;

    let map = &wolf_world.map;
    // spawn walls
    for y in 0..map.height {
        for x in 0..map.width {
            if let Some(tile) = map.walls.get(y as usize, x as usize).unwrap() {
                let tile = map.tileset.get(tile).unwrap();
                commands.spawn(WolfTileBundle::new(x,y, tile));
            }
        }
    }
    
    // spawn things
}

pub fn debug_gizmos_system(mut gizmos: Gizmos, _time: Res<Time>) {
    // draw origin
    gizmos.ray((0.0, 0.0, 0.0).into(), (0.0, 0.0, 1.0).into(), Color::BLUE);
    gizmos.ray((0.0, 0.0, 0.0).into(), (0.0, 1.0, 0.0).into(), Color::GREEN);
    gizmos.ray((0.0, 0.0, 0.0).into(), (1.0, 0.0, 0.0).into(), Color::RED);
}

pub fn build_systems(app: &mut App) {
    app.add_systems(Startup, startup_system);
    app.add_systems(PreUpdate, load_map_system);
    app.add_systems(Update, (spawn_cam_system).chain());
    app.add_systems(PostUpdate, debug_gizmos_system);
}
