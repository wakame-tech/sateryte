use bevy::prelude::*;

use crate::geo::size::Size;

use super::tile::Tile;

#[derive(Component, Debug, Clone)]
pub struct Map {
    pub tiles: Vec<Vec<Tile>>,
    pub size: Size,
}

impl Map {
    pub fn new(size: Size) -> Self {
        let map: Vec<Vec<Tile>> = vec![vec![Tile::Wall; size.w]; size.h];
        Self { tiles: map, size }
    }
}
