use bevy::prelude::*;

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
pub struct WolfCamera {
}

#[derive(Component, Default, Clone)]
pub struct WolfThing {
    pub pos:Vec3,
}

#[derive(Component, Default, Clone)]
pub struct WolfTile {
    pub pos:UVec2,
    pub texture:String
}

#[derive(Component, Default, Clone)]
pub struct WolfSprite {
    pub texture:String
}

#[derive(Component, Default, Clone)]
pub struct WolfEntity;


#[derive(Component)]
pub struct WolfUIFPSText;