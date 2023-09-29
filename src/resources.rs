use bevy::{
    prelude::*,
    reflect::{TypePath, TypeUuid},
    utils::HashMap, render::mesh::Indices,
};

use crate::WolfMap;


pub struct WolfGrid {
    spatial:flat_spatial::Grid<(Entity, Vec2), [f32;2]>, 
    map:HashMap<Entity, flat_spatial::grid::GridHandle>
}

impl Default for WolfGrid {
    fn default() -> Self {
        Self { 
            spatial:flat_spatial::Grid::new(8),
            map:HashMap::new()
        }
    }
}

impl WolfGrid {
    pub fn clear(&mut self) {
        let _ = self.spatial.clear();
        self.map.clear();
    }

    pub fn insert_or_replace(&mut self, entity:Entity, pos:Vec2) {
        if let Some(handle) = self.map.remove(&entity) {
            self.spatial.remove_maintain(handle);
        }

        let h = self.spatial.insert([pos.x, pos.y], (entity, pos));
        self.map.insert(entity, h);
    }

    pub fn query_around(&self, pos:Vec2, radius:f32) -> impl Iterator<Item = (Entity, Vec2)> + '_ {
        let iter = self.spatial.query_around([pos.x, pos.y], radius);
        let iter = iter.map(|x|*self.spatial.get(x.0).unwrap().1);
        iter
    }
}


#[derive(Default, Resource)]
pub struct WolfWorld {
    pub updates:u64,
    pub map: WolfMap,
    pub grid: WolfGrid,
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
    pub sprite_meshes:WolfAtlaseMeshes
}

#[derive(Default)]
pub struct WolfAtlaseMeshes {
    atlas_meshes:HashMap<(u8, u8), WolfSpriteMesh>
}

impl WolfAtlaseMeshes {
    pub fn get(&mut self, atlas_height:u8, atlas_width:u8, assets_mesh:&mut Assets<Mesh>) -> &WolfSpriteMesh {
        if self.atlas_meshes.contains_key(&(atlas_height, atlas_width)) == false {
            let mut meshes = Vec::new();
            for y in 0..atlas_height {
                for x in 0..atlas_width {
                    let mut mesh = Mesh::new(bevy::render::render_resource::PrimitiveTopology::TriangleList);
                    let s = 0.5;
                    let a = 1.0 / 1024.0;
                    let w = 1.0 / atlas_width as f32;
                    let h = 1.0 / atlas_height as f32;
                    let u = x as f32 * w;
                    let v = y as f32 * h;
                    
                    mesh.set_indices(Some(Indices::U16(vec![0, 1, 2, 0, 2, 3])));
                    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vec![[-s, s, 0.0], [-s, -s, 0.0], [s, -s, 0.0], [s, s, 0.0]]);
                    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, vec![[0.0, 0.0, 1.0], [0.0, 0.0, 1.0], [0.0, 0.0, 1.0], [0.0, 0.0, 1.0]]);
                    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, vec![[u + a, v + a], [u + a, v+h - a], [u+w - a, v+h - a], [u+w - a, v + a]]);
                    meshes.push(assets_mesh.add(mesh));
                }
            }
            let wam = WolfSpriteMesh {
                meshes
            };
            self.atlas_meshes.insert((atlas_height, atlas_width), wam);
        }

        self.atlas_meshes.get(&(atlas_height, atlas_width)).unwrap()
    }
}

pub struct WolfSpriteMesh {
    pub meshes:Vec<Handle<Mesh>>
}

impl WolfSpriteMesh {
    pub fn index(&self, index:u16) -> Handle<Mesh> {
        let index = index as usize % self.meshes.len(); 
        self.meshes.get(index).unwrap().clone()
    }
}

#[derive(Resource)]
pub struct WolfConfig {
    pub interaction_key: KeyCode,
    pub forward_key: KeyCode,
    pub backward_key: KeyCode,
    pub strife_left_key: KeyCode,
    pub strife_right_key: KeyCode,
    pub turn_speed:f32,
    pub show_dev:bool
}

impl Default for WolfConfig {
    fn default() -> Self {
        Self {
            interaction_key: KeyCode::Space,
            forward_key: KeyCode::W,
            backward_key: KeyCode::S,
            strife_left_key: KeyCode::A,
            strife_right_key: KeyCode::D,
            turn_speed:0.01,
            show_dev:false
        }
    }
}



pub fn build_resources(app: &mut App) {
    app.init_resource::<WolfWorld>();
    app.init_resource::<WolfAssets>();
    app.init_resource::<WolfConfig>();
}

