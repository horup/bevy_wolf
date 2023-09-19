use std::f32::consts::PI;

use bevy::{prelude::*, transform::commands};

use crate::components::{Cam, Spawn};

pub fn startup_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
}

pub fn spawn_cam_system(mut commands: Commands, spawns: Query<(Entity, &Spawn<Cam>), Added<Spawn<Cam>>>) {
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

pub fn debug_gizmos_system(mut gizmos: Gizmos, time: Res<Time>) {
    // draw origin
    gizmos.ray((0.0, 0.0, 0.0).into(), (0.0, 0.0, 1.0).into(), Color::BLUE);
    gizmos.ray((0.0, 0.0, 0.0).into(), (0.0, 1.0, 0.0).into(), Color::GREEN);
    gizmos.ray((0.0, 0.0, 0.0).into(), (1.0, 0.0, 0.0).into(), Color::RED);
}

pub fn build(app: &mut App) {
    app.add_systems(Startup, startup_system);
    app.add_systems(Update, spawn_cam_system);
    app.add_systems(PostUpdate, debug_gizmos_system);
}
