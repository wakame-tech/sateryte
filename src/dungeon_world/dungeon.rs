use bevy::prelude::*;

use crate::{
    geo::{direction::Direction, point::Point},
    world::components::tile::Tile,
};

use super::region::Region;

#[derive(Component, Debug, Clone)]
pub struct Dungeon {
    /// 地形タイル
    pub tiles: Vec<Vec<Tile>>,
    /// 部屋情報
    pub areas: Vec<Region>,
}

impl Dungeon {
    pub fn is_movable(&self, pos: &Point) -> bool {
        self.tiles[*pos].is_through()
    }

    /// 移動可能であれば, 移動先の座標を返す
    pub fn get_next_pos(&self, pos: Point, dir: &Direction) -> Option<Point> {
        let next_pos = pos + Point::from_dir(dir);
        if self.is_movable(&next_pos) {
            return Some(next_pos);
        }
        None
    }

    /// 壁に当たるまでの道を返す
    pub fn points_iter(&self, pos: Point, dir: &Direction) -> impl Iterator<Item = Point> {
        let mut pos = pos;
        let mut res: Vec<Point> = vec![];
        loop {
            if let Some(next_pos) = self.get_next_pos(pos, dir) {
                res.push(next_pos);
                pos = next_pos;
            } else {
                break;
            }
        }
        res.into_iter()
    }

    /// ダッシュを終了するかの判定
    pub fn cancel_dash(&self, pos: &Point, dir: &Direction) -> bool {
        if self.tiles[*pos] == Tile::Passage {
            // 部屋の入口か
            if self.tiles[*pos] != Tile::Passage {
                return true;
            }
            // 左右に分岐があるか
            let sides = dir
                .sides()
                .iter()
                .map(|d| pos.clone() + Point::from_dir(d))
                .collect::<Vec<_>>();
            if sides.iter().any(|p| self.tiles[*p] == Tile::Passage) {
                return true;
            }
        } else {
            // 周囲に通路があるか
            if pos
                .around4()
                .iter()
                .any(|p| self.tiles[*p] == Tile::Passage)
            {
                return true;
            }
        }
        // それ以外のときは壁までダッシュ
        let next = pos.clone() + Point::from_dir(dir);
        !self.is_movable(&next)
    }
}
