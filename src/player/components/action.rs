use crate::geo::direction::Direction;
use bevy::prelude::*;

/// [Player] の1または複数ターンの行動を表す
#[derive(Debug, Component)]
pub enum Action {
    /// 1マス移動する
    Walk(Direction),
    /// 壁まで移動する
    Dash(Direction),
}
