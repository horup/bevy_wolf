use bevy::{
    asset::{AssetLoader, LoadedAsset},
    prelude::*,
    reflect::{TypePath, TypeUuid}, utils::HashMap,
};
use array2d::Array2D;

#[derive(Clone, Debug)]
pub struct WolfMapEntity {
    pub name:String,
    pub pos:Vec3
}


#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct WolfMapTile {
    pub texture:String
}

#[derive(TypeUuid, TypePath, Debug, Clone)]
#[uuid = "8a6ed18a-13d6-45b1-8ba7-ede1b13500c5"]
pub struct WolfMap {
    pub entities:Vec<WolfMapEntity>,
    pub walls:Array2D<Option<u32>>,
    pub tileset:HashMap<u32, WolfMapTile>,
    pub width: u32,
    pub height: u32,
}

#[derive(Default)]
struct WolfMapAssetLoader;

struct BytesReader<'a> {
    pub bytes: &'a [u8],
}
impl<'a> tiled::ResourceReader for BytesReader<'a> {
    type Resource = &'a [u8];

    type Error = std::io::Error;

    fn read_from(
        &mut self,
        _: &std::path::Path,
    ) -> std::result::Result<Self::Resource, Self::Error> {
        Ok(self.bytes)
    }
}

impl AssetLoader for WolfMapAssetLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut bevy::asset::LoadContext,
    ) -> bevy::utils::BoxedFuture<'a, Result<(), bevy::asset::Error>> {
        Box::pin(async move {
            let mut loader = tiled::Loader::with_cache_and_reader(
                tiled::DefaultResourceCache::default(),
                BytesReader { bytes },
            );
            let tiled_map = loader.load_tmx_map(load_context.path()).unwrap();
            let width = tiled_map.width;
            let height = tiled_map.height;
            let mut walls = Array2D::filled_with(None, tiled_map.height as usize, tiled_map.width as usize);
            let mut tileset_map = HashMap::new();
            let mut entities = Vec::new();
            for layer in tiled_map.layers() {
                if let Some(tiled_tile_layer) = layer.as_tile_layer() {
                    for y in 0..height {
                        for x in 0..width {
                            if let Some(tiled_layer_tile) = tiled_tile_layer.get_tile(x as i32, y as i32) {
                                if let Some(tiled_tile) = tiled_layer_tile.get_tile() {
                                    if let Some(img) = &tiled_tile.image {
                                        if let Some(stem) = img.source.file_stem() {
                                            let wolf_tile = WolfMapTile {
                                                texture:stem.to_string_lossy().into()
                                            };
                                            let id = match tileset_map.get(&wolf_tile) {
                                                Some(id) => *id as u32,
                                                None => {
                                                    let id = tileset_map.len() as u32;
                                                    tileset_map.insert(wolf_tile, id);
                                                    id
                                                }
                                            };
                                            // swap such that y is up instead of down
                                            let y = height - y - 1;
                                            walls.set(x as usize, y as usize, Some(id)).expect("failed to set WolfMap.walls");
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                if let Some(tiled_object_layer) = layer.as_object_layer() {
                    for obj in tiled_object_layer.objects() {
                        entities.push(WolfMapEntity {
                            name: obj.name.clone(),
                            pos: Vec3::new(obj.x, obj.y, 0.0),
                        });
                    }
                }
            }

            let mut tileset = HashMap::new();
            for (tile, id) in tileset_map.drain() {
                tileset.insert(id, tile);
            }
            let wolf_map = WolfMap {
                entities,
                walls,
                tileset,
                width,
                height,
            };
            load_context.set_default_asset(LoadedAsset::new(wolf_map));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["tmx"]
    }
}

pub fn build(app: &mut App) {
    app.add_asset::<WolfMap>();
    app.init_asset_loader::<WolfMapAssetLoader>();
}
