use bevy::{app::PluginGroupBuilder, prelude::*};

use super::{
    components::{action::Action, event::PlayerActedEvent},
    systems::{
        action::{act_player, auto_dash, render_player},
        guide_line::spawn_guide_lines,
        spawn::world_spawn_player,
        status_bar::{
            render_player_dir_status, render_player_exp_status, render_player_hp_status,
            render_player_position_status,
        },
    },
};

/// プレイヤーの行動に関するプラグイン
pub struct PlayerActionPlugin;

impl Plugin for PlayerActionPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<Action>()
            .add_event::<PlayerActedEvent>()
            .add_system(world_spawn_player)
            .add_system(act_player.label("act_player"))
            .add_system(auto_dash.label("auto_dash").after("act_player"))
            .add_system(render_player.after("auto_dash"));
    }
}

/// プレイヤーの情報をステータスバーに表示するためのプラグイン
pub struct PlayerStatusBarPlugin;

impl Plugin for PlayerStatusBarPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(render_player_position_status.after("act_player"))
            .add_system(render_player_hp_status.after("act_player"))
            .add_system(render_player_exp_status.after("act_player"))
            .add_system(spawn_guide_lines.after("act_player"))
            .add_system(render_player_dir_status.after("act_player"));
    }
}

/// プレイヤーに関するプラグイン群
/// - [PlayerActionPlugin]
/// - [PlayerStatusBarPlugin]
pub struct PlayerPlugins;

impl PluginGroup for PlayerPlugins {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group.add(PlayerActionPlugin).add(PlayerStatusBarPlugin);
    }
}
