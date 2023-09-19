use std::f32::consts::PI;

use bevy::{prelude::*, transform::commands};

pub fn startup_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(10.0, -10.0, 10.0).looking_at(Vec3::ZERO, Vec3::Z),
        ..default()
    });
}

pub fn debug_gizmos_system(mut gizmos: Gizmos, time: Res<Time>) {
    // draw origin
    gizmos.ray((0.0, 0.0, 0.0).into(), (0.0, 0.0, 1.0).into(), Color::BLUE);
    gizmos.ray((0.0, 0.0, 0.0).into(), (0.0, 1.0, 0.0).into(), Color::GREEN);
    gizmos.ray((0.0, 0.0, 0.0).into(), (1.0, 0.0, 0.0).into(), Color::RED);
}

pub fn build(app: &mut App) {
    app.add_systems(Startup, startup_system);
    app.add_systems(PostUpdate, debug_gizmos_system);
}
