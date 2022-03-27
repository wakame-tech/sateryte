use bevy::prelude::{Assets, Commands, Component, Query, Res, ResMut};
use bevy_crossterm::components::{Position, Sprite, SpriteBundle, StyleMap};

use crate::geo::{direction::Direction, point::Point, size::Size};

use super::{
    generator::{DefaultGenerator, Generator},
    tile::Tile,
};

#[derive(Component, Debug, Clone)]
pub struct Map {
    pub tiles: Vec<Vec<Tile>>,
    pub size: Size,
}

#[derive(Component, Debug, Clone)]
pub struct MapItme;

impl Map {
    pub fn new(size: Size) -> Self {
        let map: Vec<Vec<Tile>> = vec![vec![Tile::Wall; size.w]; size.h];
        Self { tiles: map, size }
    }

    pub fn is_movable(&self, pos: Point) -> bool {
        self.tiles[pos.y as usize][pos.x as usize].is_through()
    }

    pub fn get_next_wall_pos(&self, pos: Point, dir: Direction) -> Point {
        let mut cur = pos;
        while self.is_movable(cur + dir.clone().into()) {
            cur += dir.clone().into();
        }
        cur
    }

    pub fn get_next_pos(&self, pos: Point, dir: Direction) -> Point {
        if self.is_movable(pos + dir.clone().into()) {
            pos + dir.clone().into()
        } else {
            pos
        }
    }
}

// TODO
pub fn startup_map_item(mut map: Query<&mut Map>, generator: Query<&DefaultGenerator>) {
    let generator = generator.single();
    let mut map = map.single_mut();
    let mut rng = rand::thread_rng();
    for region in &generator.areas {
        // spawn items
        for _ in 0..3 {
            if let Some(pos) = region.random_floor(&mut rng) {
                map.tiles[pos.y as usize][pos.x as usize] = Tile::Potion;
            }
        }
    }
}

pub fn startup_map(
    mut commands: Commands,
    mut sprites: ResMut<Assets<Sprite>>,
    mut stylemaps: ResMut<Assets<bevy_crossterm::components::StyleMap>>,
) {
    let mut map = Map::new(Size::new(80, 24));
    let mut generator = DefaultGenerator::new();
    generator.generate(&mut map);
    commands.spawn().insert(generator);

    let color = stylemaps.add(StyleMap::default());
    for (y, row) in map.tiles.iter().enumerate() {
        for (x, tile) in row.iter().enumerate() {
            let tile = sprites.add(tile.clone().into());
            commands.spawn_bundle(SpriteBundle {
                sprite: tile,
                position: Position::with_xy(x as i32, y as i32),
                stylemap: color.clone(),
                ..Default::default()
            });
        }
    }
}
