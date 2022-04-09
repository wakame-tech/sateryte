use bevy::prelude::*;

use crate::geo::size::Size;

use super::tile::Tile;

/// 画面に唯一存在するマップ
#[derive(Component, Debug, Clone)]
pub struct Floor {
    /// マップの地形データ
    pub tiles: Vec<Vec<Tile>>,
    /// フロアの大きさ
    pub size: Size,
}

impl Floor {
    pub fn new(size: Size) -> Self {
        let tiles: Vec<Vec<Tile>> = vec![vec![Tile::Wall; size.w]; size.h];
        Self { tiles, size }
    }
}
