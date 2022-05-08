use rand::prelude::ThreadRng;

use crate::geo::{direction::Direction, point::Point, rect::Rect};

/// A region of the map.
/// at most 1 room in a region
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Region {
    pub area: Rect,
    pub room: Option<Rect>,
}

impl Region {
    pub fn new(area: Rect) -> Region {
        Region {
            area: area.clone(),
            room: Some(area.shrink(2, 2).unwrap()),
        }
    }

    pub fn random_floor(&self, rng: &mut ThreadRng) -> Option<Point> {
        self.room
            .as_ref()
            .map(|r| r.shrink(1, 1).unwrap().sample(rng))
            .map(|p| Point::new(p.w as i32, p.h as i32))
    }

    pub fn random_wall(&self, dir: &Direction, rng: &mut ThreadRng) -> Option<Point> {
        self.room
            .as_ref()
            .map(|room| room.sample_edge(dir, rng))
            .unwrap_or(None)
    }
}
