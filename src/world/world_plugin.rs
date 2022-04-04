use bevy::prelude::*;

use crate::{
    enemy::{enemy_move, world_spawn_enemy, EnemyMovedEvent},
    player::player_plugin::PlayerPlugins,
};

use super::{
    components::event::{ItemSpawnEvent, WorldGenerateEvent},
    systems::{
        generate::{listen_world_generated, spawn_floor},
        spawn_items::spawn_items,
        turn::{increment_turn, render_turn_status, setup_turn},
    },
};

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<WorldGenerateEvent>()
            .add_event::<ItemSpawnEvent>()
            .add_event::<EnemyMovedEvent>()
            .add_plugins(PlayerPlugins)
            .add_system(setup_turn)
            .add_system(increment_turn)
            .add_system(render_turn_status)
            .add_system(spawn_floor.label("spawn_floor"))
            .add_system(listen_world_generated.after("spawn_floor"))
            .add_system(world_spawn_enemy.after("spawn_floor"))
            .add_system(spawn_items.after("spawn_floor"))
            .add_system(enemy_move.label("enemy_move").after("act_player"));
    }
}
