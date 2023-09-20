use bevy::prelude::*;

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

pub fn build_resources(app:&mut App) {
    app.insert_resource(WolfWorld::default());
}