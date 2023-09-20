use bevy::{prelude::{*, PluginGroup}, utils::HashMap, reflect::{TypeUuid, TypePath}};

use crate::WolfMap;

#[derive(Default, Resource)]
pub struct WolfWorld {
    pub map:WolfMap,
    pub(crate) map_handle:Option<Handle<WolfMap>>
}

impl WolfWorld {
    pub fn load_map(&mut self, handle:Handle<WolfMap>) {
        self.map_handle = Some(handle);
    }
}

pub struct AssetMap<T:TypeUuid + TypePath + Send + Sync> {
    assets:HashMap<String, Handle<T>>,
}

impl<T:TypeUuid + TypePath + Send + Sync> Default for AssetMap<T> {
    fn default() -> Self {
        Self { assets: Default::default() }
    }
}

impl<T:TypeUuid + TypePath + Send + Sync> AssetMap<T> {
    pub fn add(&mut self, name:&str, handle:Handle<T>) {
        self.assets.insert(name.to_string(), handle);
    }
    pub fn get(&self, name:&str) -> Option<Handle<T>> {
        let Some(h) = self.assets.get(name) else {
            return None;
        };

        Some(h.clone())
    }
}

#[derive(Resource, Default)]
pub struct WolfAssets {
    pub meshes:AssetMap<Mesh>,
    pub standard_materials:AssetMap<StandardMaterial>
}

pub fn build_resources(app:&mut App) {
    app.init_resource::<WolfWorld>();
    app.init_resource::<WolfAssets>();
}
