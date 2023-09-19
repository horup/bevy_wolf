use bevy::prelude::*;

pub enum SpawnVariant {
    Cam { cam: Cam },
}

#[derive(Component)]
pub struct Spawn {
    pub variant: SpawnVariant,
}

#[derive(Component, Clone, Default)]
pub struct Cam {
    pub pos: Vec3,
    pub yaw: f32,
}
