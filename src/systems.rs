use crate::{components::{Cam, Map, Spawn}, assets::TMXMap};
use bevy::prelude::*;

pub fn startup_system(
    _commands: Commands,
    _meshes: ResMut<Assets<Mesh>>,
    _materials: ResMut<Assets<StandardMaterial>>,
) {
}

pub fn spawn_cam_system(
    mut commands: Commands,
    spawns: Query<(Entity, &Spawn<Cam>), Added<Spawn<Cam>>>,
) {
    for (e, spawn) in spawns.iter() {
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
    }
}

pub fn spawn_map_system(
    mut commands: Commands,
    mut spawns: Query<(Entity, &Spawn<Map>)>,
    ass: Res<AssetServer>,
) {
    for (e, spawn) in spawns.iter() {
        let handle:Handle<TMXMap> = ass.load(&spawn.variant.map_path);
    }
}

pub fn debug_gizmos_system(mut gizmos: Gizmos, _time: Res<Time>) {
    // draw origin
    gizmos.ray((0.0, 0.0, 0.0).into(), (0.0, 0.0, 1.0).into(), Color::BLUE);
    gizmos.ray((0.0, 0.0, 0.0).into(), (0.0, 1.0, 0.0).into(), Color::GREEN);
    gizmos.ray((0.0, 0.0, 0.0).into(), (1.0, 0.0, 0.0).into(), Color::RED);
}

pub fn build(app: &mut App) {
    app.add_systems(Startup, startup_system);
    app.add_systems(Update, (spawn_map_system, spawn_cam_system).chain());
    app.add_systems(PostUpdate, debug_gizmos_system);
}
