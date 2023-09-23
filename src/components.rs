use bevy::{prelude::*, utils::HashMap};

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
}

#[derive(Component, Default, Clone, Debug)]
pub struct WolfEntity {
    pub image: String,
    pub classes: HashMap<String, ()>,
    pub pos: Vec3,
    pub index: UVec2
}

impl WolfEntity {
    pub fn has_class(&self, class: &str) -> bool {
        self.classes.contains_key(class)
    }
}


#[derive(Component)]
pub struct WolfUIFPSText;