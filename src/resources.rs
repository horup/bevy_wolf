use bevy::{
    prelude::*,
    reflect::{TypePath, TypeUuid},
    utils::HashMap, render::mesh::Indices,
};

use crate::WolfMap;

#[derive(Default, Resource)]
pub struct WolfWorld {
    pub updates:u64,
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
    pub atlas_meshes:WolfAtlaseMeshes
}

#[derive(Default)]
pub struct WolfAtlaseMeshes {
    atlas_meshes:HashMap<(u8, u8), WolfAtlasMesh>
}

impl WolfAtlaseMeshes {
    pub fn get(&mut self, atlas_height:u8, atlas_width:u8, assets_mesh:&mut Assets<Mesh>) -> &WolfAtlasMesh {
        if self.atlas_meshes.contains_key(&(atlas_height, atlas_width)) == false {
            let mut meshes = Vec::new();
            for y in 0..atlas_height {
                for x in 0..atlas_width {
                    let mut mesh = Mesh::new(bevy::render::render_resource::PrimitiveTopology::TriangleList);
                    let s = 0.5;
                    let w = 1.0 / atlas_width as f32;
                    let h = 1.0 / atlas_height as f32;
                    let u = x as f32 * w;
                    let v = y as f32 * h;
                    mesh.set_indices(Some(Indices::U16(vec![0, 1, 2, 0, 2, 3])));
                    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vec![[-s, s, 0.0], [-s, -s, 0.0], [s, -s, 0.0], [s, s, 0.0]]);
                    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, vec![[0.0, 0.0, 1.0], [0.0, 0.0, 1.0], [0.0, 0.0, 1.0], [0.0, 0.0, 1.0]]);
                    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, vec![[u, v], [u, v+h], [u+w, v+h], [u+w, v]]);
                    meshes.push(assets_mesh.add(mesh));
                }
            }
            let wam = WolfAtlasMesh {
                meshes
            };
            self.atlas_meshes.insert((atlas_height, atlas_width), wam);
        }

        self.atlas_meshes.get(&(atlas_height, atlas_width)).unwrap()
    }
}

pub struct WolfAtlasMesh {
    pub meshes:Vec<Handle<Mesh>>
}

impl WolfAtlasMesh {
    pub fn index(&self, index:u16) -> Option<Handle<Mesh>> {
        if let Some(h) = self.meshes.get(index as usize) {
            return Some(h.clone());
        }

        None
    }
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
