use bevy::prelude::*;

use crate::{WolfEntity, WolfMapTile, WolfTile};

#[derive(Bundle, Default)]
pub struct WolfTileBundle {
    pub wolf_tile: WolfTile,
    pub wolf_entity: WolfEntity,
}

impl WolfTileBundle {
    pub fn new(x: u32, y: u32, tile: &WolfMapTile) -> Self {
        Self {
            wolf_tile: WolfTile {
                pos: UVec2::new(x, y),
                texture:tile.texture.clone()
            },
            ..Default::default()
        }
    }
}
