use bevy::{app::PluginGroupBuilder, prelude::*};

use crate::{
    enemy::enemy_plugin::EnemyPlugins, player::player_plugin::PlayerPlugins,
    turn::turn_plugin::TurnPlugin,
};

use super::{
    components::event::{ItemSpawnEvent, WorldGenerateEvent},
    systems::{
        generate::{listen_world_generated, spawn_floor},
        spawn_items::spawn_items,
    },
};

/// ワールドの生成に関するプラグイン
pub struct FloorGeneratorPlugin;

impl Plugin for FloorGeneratorPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<WorldGenerateEvent>()
            .add_system(spawn_floor.label("spawn_floor"))
            .add_system(listen_world_generated.after("spawn_floor"));
    }
}

/// アイテムのスポーンに関するプラグイン
pub struct FloorItemSpawnPlugin;

impl Plugin for FloorItemSpawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ItemSpawnEvent>()
            .add_system(spawn_items.after("spawn_floor"));
    }
}

/// マップの生成にかかわるプラグイン群
/// - [FloorGeneratorPlugin]
/// - [FloorItemSpawnPlugin]
pub struct WorldGeneratorPlugins;

impl PluginGroup for WorldGeneratorPlugins {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group.add(FloorGeneratorPlugin).add(FloorItemSpawnPlugin);
    }
}
/// フロアに関わるプラグイン
pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PlayerPlugins)
            .add_plugins(WorldGeneratorPlugins)
            .add_plugin(TurnPlugin)
            .add_plugins(EnemyPlugins);
    }
}
