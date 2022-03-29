use bevy::prelude::*;
use bevy_crossterm::components::{Position, Sprite, SpriteBundle, StyleMap};

use crate::geo::size::Size;

use super::{
    generator::{DungeonGenerator, Generator},
    tile::{tile_style, Tile},
};

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

pub fn generate_map(
    mut commands: Commands,
    mut sprites: ResMut<Assets<Sprite>>,
    mut stylemaps: ResMut<Assets<bevy_crossterm::components::StyleMap>>,
) {
    let mut map = Map::new(Size::new(80, 24));
    let mut generator = DungeonGenerator;
    let dungeon = generator.generate(&mut map);
    commands.spawn().insert(dungeon);

    for (y, row) in map.tiles.iter().enumerate() {
        for (x, tile) in row.iter().enumerate() {
            let stylemap = stylemaps.add(StyleMap::new(tile_style(tile), vec![]));
            let tile = sprites.add(tile.clone().into());
            commands.spawn_bundle(SpriteBundle {
                sprite: tile,
                position: Position::with_xy(x as i32, y as i32),
                stylemap,
                ..Default::default()
            });
        }
    }
}
