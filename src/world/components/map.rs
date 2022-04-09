use bevy::prelude::*;

use crate::geo::size::Size;

use super::tile::Tile;

#[derive(Component, Debug, Clone)]
pub struct Map {
    /// マップの地形データ
    pub tiles: Vec<Vec<Tile>>,
    /// マップのオーバーレイ
    pub overlays: Vec<Vec<Tile>>,
    /// フロアの大きさ
    pub size: Size,
}

impl Map {
    pub fn new(size: Size) -> Self {
        let tiles: Vec<Vec<Tile>> = vec![vec![Tile::Wall; size.w]; size.h];
        let overlays: Vec<Vec<Tile>> = vec![vec![Tile::Debug(" ".to_owned()); size.w]; size.h];
        Self {
            tiles,
            overlays,
            size,
        }
    }
}
