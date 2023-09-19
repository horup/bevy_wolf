
use bevy::{
    asset::AssetLoader,
    prelude::*,
    reflect::{TypePath, TypeUuid},
};
use tiled::Loader;

#[derive(TypeUuid, TypePath)]
#[uuid = "8a6ed18a-13d6-45b1-8ba7-ede1b13500c5"]
pub struct TMXMap {
    pub map: tiled::Map,
}

#[derive(Default)]
struct TMXAssetLoader;

struct BytesReader<'a> {
    pub bytes: &'a [u8],
}
impl<'a> tiled::ResourceReader for BytesReader<'a> {
    type Resource = &'a [u8];

    type Error = std::io::Error;

    fn read_from(&mut self, path: &std::path::Path) -> std::result::Result<Self::Resource, Self::Error> {
        Ok(self.bytes)
    }
}

impl AssetLoader for TMXAssetLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut bevy::asset::LoadContext,
    ) -> bevy::utils::BoxedFuture<'a, Result<(), bevy::asset::Error>> {
        Box::pin(async move {
            let mut bytes_reader = BytesReader { bytes };
            let mut loader = 
            tiled::Loader::with_cache_and_reader(
                tiled::DefaultResourceCache::default(),
                bytes_reader,
            );
            let map = loader.load_tmx_map(load_context.path()).unwrap();
            dbg!(map);
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["tmx"]
    }
}

pub fn build(app: &mut App) {
    app.add_asset::<TMXMap>();
    app.init_asset_loader::<TMXAssetLoader>();
}
