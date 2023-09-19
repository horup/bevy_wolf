use bevy::{
    asset::AssetLoader,
    prelude::*,
    reflect::{TypePath, TypeUuid},
};

#[derive(TypeUuid, TypePath, Debug, Clone)]
#[uuid = "8a6ed18a-13d6-45b1-8ba7-ede1b13500c5"]
pub struct WolfMap {
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

            let wmap = WolfMap {
                width: tiled_map.width,
                height: tiled_map.height,
            };
            dbg!(&wmap);
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
