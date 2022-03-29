use bevy::prelude::*;

use crate::geo::{direction::Direction, point::Point};

use super::{region::Region, tile::Tile};

#[derive(Component, Debug)]
pub struct Dungeon {
    pub tiles: Vec<Vec<Tile>>,
    pub areas: Vec<Region>,
}

impl Dungeon {
    pub fn is_throughable(&self, pos: Point) -> bool {
        self.tiles[pos.y as usize][pos.x as usize].is_through()
    }

    pub fn is_movable(&self, pos: Point) -> bool {
        if pos.x < 0
            || pos.y < 0
            || pos.x >= self.tiles[0].len() as i32
            || pos.y >= self.tiles.len() as i32
        {
            return false;
        }
        self.tiles[pos.y as usize][pos.x as usize].is_through()
    }

    pub fn get_next_wall_pos(&self, pos: Point, dir: Direction) -> Point {
        let mut cur = pos;
        let diff = dir.clone().into();
        while self.is_movable(cur + diff) {
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
