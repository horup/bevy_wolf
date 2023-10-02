use std::ops::{Deref, DerefMut};
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

#[derive(Component, Clone)]
pub struct Prev<T:Component + Clone> {
    pub component:T
}

impl<T:Component + Clone> Deref for Prev<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.component
    }
}

impl<T:Component + Clone> DerefMut for Prev<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.component
    }
}

pub const BODY_SHAPE_CUBOID:u8 = 0;
pub const BODY_SHAPE_BALL:u8 = 1;

#[derive(Component, Clone, Debug)]
pub struct WolfBody {
    pub height:f32,
    pub radius:f32,
    pub shape:u8,
    pub disabled:bool
}

impl Default for WolfBody {
    fn default() -> Self {
        Self { 
            height:1.0,
            radius:0.5,
            shape:BODY_SHAPE_CUBOID,
            disabled: false
        }
    }
}

#[derive(Component, Clone, Default)]
pub struct WolfCamera {
}

#[derive(Component, Default, Clone)]
pub struct WolfSprite {
    pub index:f32,
    pub atlas_width:u8,
    pub atlas_height:u8
}

#[derive(Component, Clone)]
pub struct WolfEntityRef {
    pub entity:Entity
}

#[derive(Component, Default, Clone, Debug)]
pub struct WolfEntity {
    pub(crate) classes: Vec<String>,
    pub(crate) properties_float: HashMap<String, f32>,
    pub(crate) properties_int: HashMap<String, i32>,
    pub(crate) properties_string: HashMap<String, String>,
    pub(crate) start_pos: Vec3,
    pub(crate) start_facing: f32
}

impl WolfEntity {
    pub fn start_pos(&self) -> Vec3 {
        self.start_pos
    }

    pub fn start_facing(&self) -> f32 {
        self.start_facing
    }

    pub fn has_class(&self, class: &str) -> bool {
        for c in self.classes.iter() {
            if c == class {
                return true;
            }
        }

        return false;
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


#[derive(Component, Default)]
pub struct WolfInteract {
    
}

#[derive(Default, Clone)]
pub struct Timer {
    pub current:f32,
    pub start:f32
}

impl Timer {
    pub fn start(secs:f32) -> Self {
        Self {
            current:secs,
            start:secs
        }
    }

    pub fn tick(&mut self, dt_sec:f32) {
        if self.current > 0.0 {
            self.current -= dt_sec;
            if self.current <= 0.0 {
                self.current = 0.0;
            }
        }
    }

    pub fn is_done(&self) -> bool {
        self.current == 0.0
    }

    pub fn alpha(&self) -> f32 {
        let a = 1.0 - self.current / self.start;
        a.clamp(0.0, 1.0)
    }
}

pub enum DoorState {
    Closed,
    Closing {
        closing:Timer
    },
    Opening {
        opening:Timer
    },
    Open {
        auto_close_timer:Timer
    }
}

impl Default for DoorState {
    fn default() -> Self {
        Self::Closed
    }
}

#[derive(Component, Default)]
pub struct WolfDoor {
    pub pos:Vec3,
    pub state:DoorState
}

impl WolfDoor {
   
}