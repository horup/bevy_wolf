use bevy::{asset::AssetLoader, prelude::*, reflect::{TypeUuid, TypePath}};
use tiled::Loader;

#[derive(TypeUuid, TypePath)]
#[uuid = "8a6ed18a-13d6-45b1-8ba7-ede1b13500c5"]
pub struct TMXMap {
    pub map:tiled::Map
}

#[derive(Default)]
struct TMXAssetLoader;

impl AssetLoader for TMXAssetLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut bevy::asset::LoadContext,
    ) -> bevy::utils::BoxedFuture<'a, Result<(), bevy::asset::Error>> {
        Box::pin(async move {
            let mut loader = Loader::new();
            load_context.
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
