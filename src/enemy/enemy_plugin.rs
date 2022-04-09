use bevy::{app::PluginGroupBuilder, prelude::*};

use super::{
    components::EnemyMovedEvent,
    systems::{move_enemy::enemy_move, spawn_enemy::world_spawn_enemy},
};

/// 敵に関するプラグイン
struct EnemyActionPlugin;

impl Plugin for EnemyActionPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<EnemyMovedEvent>()
            .add_system(world_spawn_enemy.after("spawn_floor"))
            .add_system(enemy_move.label("enemy_move").after("act_player"));
    }
}

/// 敵に関するプラグイン群
pub struct EnemyPlugins;

impl PluginGroup for EnemyPlugins {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group.add(EnemyActionPlugin);
    }
}
