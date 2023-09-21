use bevy::{
    prelude::{PluginGroup, *},
    reflect::{TypePath, TypeUuid},
    utils::HashMap,
};

use crate::WolfMap;

#[derive(Default, Resource)]
pub struct WolfWorld {
    pub map: WolfMap,
    pub(crate) map_handle: Option<Handle<WolfMap>>,
}

impl WolfWorld {
    pub fn load_map(&mut self, handle: Handle<WolfMap>) {
        self.map_handle = Some(handle);
    }
}

pub struct AssetMap<T: TypeUuid + TypePath + Send + Sync> {
    assets: HashMap<String, Handle<T>>,
}

impl<T: TypeUuid + TypePath + Send + Sync> Default for AssetMap<T> {
    fn default() -> Self {
        Self {
            assets: Default::default(),
        }
    }
}

impl<T: TypeUuid + TypePath + Send + Sync> AssetMap<T> {
    pub fn insert(&mut self, name: &str, handle: Handle<T>) {
        self.assets.insert(name.to_string(), handle);
    }
    pub fn has(&self, name: &str) -> bool {
        self.assets.contains_key(name)
    }
    pub fn get(&self, name: &str) -> Option<Handle<T>> {
        let Some(h) = self.assets.get(name) else {
            return None;
        };

        Some(h.clone())
    }
}

#[derive(Resource, Default)]
pub struct WolfAssets {
    pub meshes: AssetMap<Mesh>,
    pub standard_materials: AssetMap<StandardMaterial>,
    pub images: AssetMap<Image>,
}

#[derive(Resource)]
pub struct WolfConfig {
    pub forward_key: KeyCode,
    pub backward_key: KeyCode,
    pub strife_left_key: KeyCode,
    pub strife_right_key: KeyCode,
    pub turn_speed:f32,
}

impl Default for WolfConfig {
    fn default() -> Self {
        Self {
            forward_key: KeyCode::W,
            backward_key: KeyCode::S,
            strife_left_key: KeyCode::A,
            strife_right_key: KeyCode::D,
            turn_speed:0.01
        }
    }
}

pub fn build_resources(app: &mut App) {
    app.init_resource::<WolfWorld>();
    app.init_resource::<WolfAssets>();
    app.init_resource::<WolfConfig>();
}
