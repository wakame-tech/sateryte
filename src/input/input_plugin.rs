use bevy::prelude::*;

use super::input_keys::input_keys;

/// キーボード入力
pub struct KeyBoardInputPlugin;

impl Plugin for KeyBoardInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(input_keys);
    }
}
