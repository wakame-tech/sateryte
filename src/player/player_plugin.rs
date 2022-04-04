use bevy::{app::PluginGroupBuilder, prelude::*};

use super::{
    components::{action::Action, event::PlayerMoveEvent},
    systems::{
        action::act_player,
        spawn::world_spawn_player,
        status_bar::{
            render_player_exp_status, render_player_hp_status, render_player_position_status,
        },
    },
};

/// プレイヤーの行動に関するプラグイン
pub struct PlayerActionPlugin;

impl Plugin for PlayerActionPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<Action>()
            .add_event::<PlayerMoveEvent>()
            .add_system(world_spawn_player)
            .add_system(act_player);
    }
}

/// プレイヤーの情報をステータスバーに表示するためのプラグイン
pub struct PlayerStatusBarPlugin;

impl Plugin for PlayerStatusBarPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(render_player_position_status)
            .add_system(render_player_hp_status)
            .add_system(render_player_exp_status);
    }
}

/// プレイヤーに関するプラグイン群
pub struct PlayerPlugins;

impl PluginGroup for PlayerPlugins {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group.add(PlayerActionPlugin).add(PlayerStatusBarPlugin);
    }
}
