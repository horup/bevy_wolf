use bevy::prelude::*;

#[derive(Event)]
pub struct WolfInteractEvent {
    pub interactor:Entity, 
    pub entity:Entity
}

pub fn build_events(app:&mut App) {
    app.add_event::<WolfInteractEvent>();
}