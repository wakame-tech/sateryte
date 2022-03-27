use rand::prelude::ThreadRng;

use crate::geo::{point::Point, rect::Rect, size::Size};

use super::{map::Map, tile::Tile};

#[derive(Debug)]
pub struct Region {
    pub area: Rect,
    pub room: Option<Rect>,
}

impl Region {
    pub fn new(area: Rect) -> Region {
        Region {
            area: area.clone(),
            room: Some(area.shrink((2, 2).into()).clone()),
        }
    }

    pub fn random_floor(&self, rng: &mut ThreadRng) -> Option<Point> {
        self.room
            .as_ref()
            .map(|r| r.shrink(Size::new(1, 1)).sample(rng))
            .map(|p| Point::new(p.w as i32, p.h as i32))
    }

    pub fn draw(&self, map: &mut Map) {
        for y in self.area.pos.y..self.area.pos.y + self.area.size.h as i32 {
            for x in self.area.pos.x..self.area.pos.x + self.area.size.w as i32 {
                if self.area.is_edge(Point::new(x, y)) {
                    map.tiles[y as usize][x as usize] = Tile::Region;
                }
            }
        }

        if let Some(room) = &self.room {
            for y in room.pos.y..(room.pos.y + room.size.h as i32) {
                for x in room.pos.x..(room.pos.x + room.size.w as i32) {
                    map.tiles[y as usize][x as usize] = Tile::Floor;
                    if room.is_vertical_edge((x, y).into()) {
                        map.tiles[y as usize][x as usize] = Tile::WallV;
                    }
                    if room.is_horizontal_edge((x, y).into()) {
                        map.tiles[y as usize][x as usize] = Tile::WallH;
                    }
                }
            }
        }
    }
}
