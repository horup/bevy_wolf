use array2d::Array2D;
use bevy::{
    asset::{AssetLoader, LoadedAsset},
    prelude::*,
    reflect::{TypePath, TypeUuid},
    utils::HashMap,
};

#[derive(Clone, Debug)]
pub struct WolfMapEntity {
    pub name: String,
    pub classes: HashMap<String, ()>,
    pub pos: Vec3,
}

impl WolfMapEntity {
    pub fn has_class(&self, class:&str) -> bool {
        self.classes.contains_key(class)
    }
}


#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct WolfMapTile {
    pub texture: String,
}

#[derive(TypeUuid, TypePath, Debug, Clone)]
#[uuid = "8a6ed18a-13d6-45b1-8ba7-ede1b13500c5"]
pub struct WolfMap {
    pub entities: Vec<WolfMapEntity>,
    pub blocks: Array2D<Option<u32>>,
    pub tileset: HashMap<u32, WolfMapTile>,
    pub width: u32,
    pub height: u32,
}

impl Default for WolfMap {
    fn default() -> Self {
        Self {
            entities: Default::default(),
            blocks: Array2D::filled_with(None, 1, 1),
            tileset: Default::default(),
            width: Default::default(),
            height: Default::default(),
        }
    }
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
            let mut blocks =
                Array2D::filled_with(None, tiled_map.height as usize, tiled_map.width as usize);
            let mut tileset_map = HashMap::new();
            let mut entities = Vec::new();
            for layer in tiled_map.layers() {
                if let Some(tiled_tile_layer) = layer.as_tile_layer() {
                    for y in 0..height {
                        for x in 0..width {
                            if let Some(tiled_layer_tile) =
                                tiled_tile_layer.get_tile(x as i32, y as i32)
                            {
                                if let Some(tiled_tile) = tiled_layer_tile.get_tile() {
                                    if let Some(img) = &tiled_tile.image {
                                        if let Some(stem) = img.source.file_stem() {
                                            let stem = stem.to_string_lossy().to_string();
                                            let y = height - y - 1;
                                            match &tiled_tile.user_type {
                                                Some(user_type) => {
                                                    let x = x as f32 + 0.5;
                                                    let y = y as f32 + 0.5;
                                                    let mut classes = HashMap::new();
                                                    for s in user_type.split(" ") {
                                                        classes.insert(s.to_string(), ());
                                                    }
                                                    entities.push(WolfMapEntity {
                                                        name: stem,
                                                        classes: classes,
                                                        pos: Vec3::new(x, y, 0.0),
                                                    });
                                                }
                                                None => {
                                                    let wolf_tile = WolfMapTile { texture: stem };

                                                    let id = match tileset_map.get(&wolf_tile) {
                                                        Some(id) => *id as u32,
                                                        None => {
                                                            let id = tileset_map.len() as u32;
                                                            tileset_map.insert(wolf_tile, id);
                                                            id
                                                        }
                                                    };
                                                    // swap such that y is up instead of down
                                                    
                                                    blocks
                                                        .set(y as usize, x as usize, Some(id))
                                                        .expect("failed to set WolfMap blocks");
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                /*if let Some(tiled_object_layer) = layer.as_object_layer() {
                    for obj in tiled_object_layer.objects() {
                        let mut pos = Vec3::new(obj.x, obj.y, 0.0);
                        pos.y = (tiled_map.height * tiled_map.tile_height) as f32 - pos.y;
                        let mut size = Vec2::new(0.0, 0.0);
                        match obj.shape.clone() {
                            tiled::ObjectShape::Rect { width, height } => {
                                size = (width, height).into()
                            }
                            tiled::ObjectShape::Ellipse { width, height } => {
                                size = (width, height).into()
                            }
                            _ => {}
                        }
                        pos.x += size.x / 2.0;
                        pos.y += size.y / 2.0;
                        pos.x /= tiled_map.tile_width as f32;
                        pos.y /= tiled_map.tile_height as f32;
                        entities.push(WolfMapEntity {
                            name: obj.name.clone(),
                            pos,
                        });
                    }
                }*/
            }

            let mut tileset = HashMap::new();
            for (tile, id) in tileset_map.drain() {
                tileset.insert(id, tile);
            }
            let wolf_map = WolfMap {
                entities,
                blocks,
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

pub fn build_assets(app: &mut App) {
    app.add_asset::<WolfMap>();
    app.init_asset_loader::<WolfMapAssetLoader>();
}
