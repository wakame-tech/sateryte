use bevy::prelude::*;

use super::{
    components::event::{ItemSpawnEvent, WorldGenerateEvent, WorldGeneratedEvent},
    systems::generate::{listen_world_generated, spawn_floor},
};

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<WorldGenerateEvent>()
            .add_event::<WorldGeneratedEvent>()
            .add_event::<ItemSpawnEvent>()
            .add_system(spawn_floor)
            .add_system(listen_world_generated);
    }
}
