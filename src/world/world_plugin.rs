use bevy::prelude::*;

use crate::enemy::{enemy_move, world_spawn_enemy, EnemyMovedEvent};

use super::{
    components::event::{ItemSpawnEvent, WorldGenerateEvent},
    systems::generate::{listen_world_generated, spawn_floor},
};

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<WorldGenerateEvent>()
            .add_event::<ItemSpawnEvent>()
            .add_event::<EnemyMovedEvent>()
            .add_system(spawn_floor)
            .add_system(listen_world_generated)
            .add_system(world_spawn_enemy)
            .add_system(enemy_move);
    }
}
