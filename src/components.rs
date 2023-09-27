use bevy::{prelude::*, utils::HashMap, asset::Asset};

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
pub struct WolfSprite {
    pub index:f32,
    pub atlas_width:u8,
    pub atlas_height:u8,
    pub offset:Vec3,
}

#[derive(Component, Default, Clone, Debug)]
pub struct WolfEntity {
    pub classes: HashMap<String, ()>,
    pub properties_float: HashMap<String, f32>,
    pub properties_int: HashMap<String, i32>,
    pub properties_string: HashMap<String, String>,
    pub pos: Vec3,
    pub index: UVec2,
    pub facing: f32
}

impl WolfEntity {
    pub fn has_class(&self, class: &str) -> bool {
        self.classes.contains_key(class)
    }

    pub fn get_property_f32(&self, property:&str) -> Option<&f32> {
        self.properties_float.get(property)
    }

    pub fn get_property_int(&self, property:&str) -> Option<&i32> {
        self.properties_int.get(property)
    }

    pub fn get_property_string(&self, property:&str) -> Option<&String> {
        self.properties_string.get(property)
    }
}

#[derive(Component, Default, Clone)]
pub struct WolfInstance<M:Material + Asset> {
    pub mesh:Handle<Mesh>,
    pub material:Handle<M>,
    pub request_redraw:bool
}

impl<M:Material + Asset> PartialEq for WolfInstance<M> {
    fn eq(&self, other: &Self) -> bool {
        self.mesh == other.mesh && self.material == other.material
    }
}

impl<M:Material + Asset> Eq for WolfInstance<M> {

}

impl<M:Material + Asset> std::hash::Hash for WolfInstance<M> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.mesh.hash(state);
        self.material.hash(state);
    }
} 

#[derive(Component)]
pub struct WolfInstanceManager<M:Material + Asset> {
    pub instance:WolfInstance<M>,
    pub request_redraw:bool,
    pub count:u32
}

#[derive(Component)]
pub struct WolfUIFPSText;