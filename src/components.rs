use bevy::prelude::*;

use crate::assets::TMXMap;

#[derive(Component, Clone)]
pub struct Spawn<T : Clone> {
    pub variant:T
}

impl<T : Clone> Spawn<T> {
    pub fn new(t:T) -> Self {
        Self { variant: t }
    }
}

#[derive(Component, Clone, Default)]
pub struct Cam {
    pub pos: Vec3,
    pub yaw: f32,
}

#[derive(Component, Default, Clone)]
pub struct Thing {
    pub pos: Vec3,
}

#[derive(Component, Default, Clone)]
pub struct Map {
    pub map_path:String,
    pub handle:Option<Handle<TMXMap>>
}