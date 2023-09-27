use array2d::Array2D;
use bevy::{
    asset::{AssetLoader, LoadedAsset},
    prelude::*,
    reflect::{TypePath, TypeUuid},
    utils::HashMap,
};
use tiled::PropertyValue;

use crate::WolfEntity;

#[derive(TypeUuid, TypePath, Debug, Clone)]
#[uuid = "8a6ed18a-13d6-45b1-8ba7-ede1b13500c5"]
pub struct WolfMap {
    pub layers: Vec<Array2D<Option<WolfEntity>>>,
    pub width: u32,
    pub height: u32,
}

impl WolfMap {
    pub fn get(&self, index:UVec2) -> Vec<&WolfEntity> {
        let mut v = Vec::new();
        for layer in self.layers.iter() {
            if let Some(tile) = layer.get(index.y as usize, index.x as usize) {
                if let Some(tile) = tile {
                    v.push(tile);
                }
            }
        }
        return v;
    }
}

impl Default for WolfMap {
    fn default() -> Self {
        Self {
            layers: Default::default(),
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
            let mut layers = Vec::new();
            for tiled_layer in tiled_map.layers() {
                if let Some(tiled_tile_layer) = tiled_layer.as_tile_layer() {
                    let mut layer = Array2D::filled_with(None, height as usize, width as usize);
                    for y in 0..height {
                        for x in 0..width {
                            if let Some(tiled_layer_tile) =
                                tiled_tile_layer.get_tile(x as i32, y as i32)
                            {
                                if let Some(tiled_tile) = tiled_layer_tile.get_tile() {
                                    let y = height - y - 1;
                                    let tile = layer.get_mut(y as usize, x as usize).unwrap();

                                    let mut classes:HashMap<String, ()> = HashMap::default();
                                    for class in tiled_tile.user_type.clone().unwrap_or_default().split(" ") {
                                        classes.insert(class.to_string(), ());
                                    }
                                    let image = tiled_tile.properties.get("image").and_then(|p|match p {
                                        tiled::PropertyValue::StringValue(s) => Some(s.clone()),
                                        _=> None
                                    }).unwrap_or_default();

                                    let mut atlas_width = 1;
                                    let mut atlas_height = 1;
                                    let mut facing = 0.0;
                                    if let Some(v) = tiled_tile.properties.get("atlas_width") {
                                        if let PropertyValue::IntValue(i) = v {
                                            atlas_width = *i as u8;
                                        }
                                    }
                                    if let Some(v) = tiled_tile.properties.get("atlas_height") {
                                        if let PropertyValue::IntValue(i) = v {
                                            atlas_height = *i as u8;
                                        }
                                    }

                                    if let Some(v) = tiled_tile.properties.get("facing") {
                                        if let PropertyValue::FloatValue(f) = v {
                                            facing = *f as f32;
                                        }
                                    }

                                    *tile = Some(WolfEntity {
                                        image,
                                        classes,
                                        atlas_width,
                                        atlas_height,
                                        pos: Vec3::new(x as f32 + 0.5, y as f32 + 0.5, 0.0),
                                        index: UVec2::new(x, y),
                                        facing,
                                    });
                                }
                            }
                        }
                    }
                    layers.push(layer);
                }
            }

            let wolf_map = WolfMap {
                layers,
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
