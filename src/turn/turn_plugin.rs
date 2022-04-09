use bevy::prelude::*;

use super::systems::{increment_turn, render_turn_status, setup_turn};

/// ターン制に関するプラグイン
pub struct TurnPlugin;

impl Plugin for TurnPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(setup_turn)
            .add_system(increment_turn)
            .add_system(render_turn_status);
    }
}
