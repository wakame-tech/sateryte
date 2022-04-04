use bevy::prelude::*;

use crate::{
    geo::{direction::Direction, point::Point},
    player::components::entity_bundle::Flags,
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

    pub fn is_movable(&self, pos: &Point) -> bool {
        self.at(pos).map(|tile| tile.is_through()).unwrap_or(false)
    }

    /// 周囲4方向に [tile] が存在するか
    fn exist_around(&self, pos: &Point, predicate: fn(&Tile) -> bool) -> bool {
        pos.around4()
            .iter()
            .any(|p| self.at(p).map_or(false, predicate))
    }

    /// 移動可能であれば, 移動先の座標を返す
    pub fn get_next_pos(&self, pos: Point, dir: &Direction) -> Option<(Point, Direction)> {
        let next_pos = pos + dir.clone().into();
        if self.is_movable(&next_pos) {
            return Some((next_pos, dir.clone()));
        }
        None
    }

    /// ダッシュを終了するかの判定
    pub fn cancel_dash(&self, pos: &Point, dir: &Direction) -> bool {
        let on_passage = self.at(pos) == Some(&Tile::Passage);
        // stops on entrance of room
        if on_passage && self.at(pos) != Some(&Tile::Passage) {
            return true;
        }
        // stops around passage
        if !on_passage && self.exist_around(pos, |tile| tile == &Tile::Passage) {
            return true;
        }
        let next = pos.clone() + dir.clone().into();
        !self.is_movable(&next)
    }
}
