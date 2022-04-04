use bevy::prelude::*;

use crate::{
    geo::{direction::Direction, point::Point},
    world::components::tile::Tile,
};

use super::region::Region;

#[derive(Component, Debug)]
pub struct Dungeon {
    pub tiles: Vec<Vec<Tile>>,
    pub areas: Vec<Region>,
}

impl Dungeon {
    pub fn at(&self, pos: &Point) -> Option<&Tile> {
        if pos.x < 0
            || pos.y < 0
            || pos.x >= self.tiles[0].len() as i32
            || pos.y >= self.tiles.len() as i32
        {
            return None;
        }
        Some(&self.tiles[pos.y as usize][pos.x as usize])
    }

    pub fn is_movable(&self, pos: Point) -> bool {
        self.at(&pos).map(|tile| tile.is_through()).unwrap_or(false)
    }

    pub fn passage_exist_around(&self, pos: Point) -> bool {
        pos.around4()
            .iter()
            .any(|p| self.at(p) == Some(&Tile::Passage))
    }

    /// Returns the next position of the wall in the given direction.
    /// if passage is found around player pos, returns here.
    pub fn get_next_wall_pos(&self, pos: Point, dir: &Direction) -> Point {
        let on_passage = self.at(&pos) == Some(&Tile::Passage);
        let mut cur = pos;
        while self.is_movable(cur + dir.clone().into()) {
            cur += dir.clone().into();
            // stops on entrance of room
            if on_passage && self.at(&cur) != Some(&Tile::Passage) {
                break;
            }
            // stops around passage
            if !on_passage && self.passage_exist_around(cur) {
                break;
            }
        }
        cur
    }

    pub fn get_next_pos(&self, pos: Point, dir: &Direction) -> Point {
        if self.is_movable(pos + dir.clone().into()) {
            pos + dir.clone().into()
        } else {
            pos
        }
    }
}
