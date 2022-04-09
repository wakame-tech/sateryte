use std::collections::HashSet;

use itertools::Itertools;
use rand::{prelude::ThreadRng, Rng};

use crate::{
    geo::{direction::Direction, point::Point, rect::Rect},
    world::components::{map::Map, tile::Tile},
};

use super::{dungeon::Dungeon, region::Region};

pub trait Generator<T> {
    fn generate(&mut self, map: &mut Map) -> T;
}

fn devide_rec(rng: &mut ThreadRng, rect: Rect, min_area_size: usize) -> Vec<Rect> {
    let mut rects: Vec<Rect> = vec![];
    let mut queue = vec![rect];

    while let Some(rect) = queue.pop() {
        if rect.size.w <= min_area_size || rect.size.h <= min_area_size {
            continue;
        }
        let res = match rng.gen_bool(0.5) {
            true => rect.devide_horizontal(rng.gen_range(min_area_size..=rect.size.w)),
            _ => rect.devide_vertical(rng.gen_range(min_area_size..=rect.size.h)),
        };
        if let Some(res) = res {
            queue.extend(res);
        } else {
            rects.push(rect);
        }
    }
    rects
}

fn build_region(map: &mut Map, region: &Region) {
    if let Some(room) = &region.room {
        for y in room.pos.y..(room.pos.y + room.size.h as i32) {
            for x in room.pos.x..(room.pos.x + room.size.w as i32) {
                map.tiles[y as usize][x as usize] = Tile::Floor;
                if room.is_vertical_edge((x, y).into()) {
                    map.tiles[y as usize][x as usize] = Tile::WallV;
                    // map.tiles[y as usize][x as usize] = Tile::Debug(i.to_string());
                }
                if room.is_horizontal_edge((x, y).into()) {
                    map.tiles[y as usize][x as usize] = Tile::WallH;
                    // map.tiles[y as usize][x as usize] = Tile::Debug(i.to_string());
                }
            }
        }
    }
}

fn get_poses_align(from: Point, to: Point, dir: Direction, wall: i32) -> HashSet<Point> {
    if dir == Direction::Up || dir == Direction::Down {
        vec![
            from.toward_v(wall),
            to.toward_v(wall),
            Point::new(from.x, wall).toward_h(to.x),
        ]
        .into_iter()
        .flatten()
        .collect()
    } else {
        vec![
            from.toward_h(wall),
            to.toward_h(wall),
            Point::new(wall, from.y).toward_v(to.y),
        ]
        .into_iter()
        .flatten()
        .collect()
    }
}

fn build_passage(map: &mut Map, from: &Region, to: &Region) {
    if from.room.is_none() || to.room.is_none() {
        return;
    }
    if let Some(dir) = from.area.intercect(&to.area) {
        let mut rng = rand::thread_rng();

        let from_room = from.room.as_ref().unwrap();
        let to_room = to.room.as_ref().unwrap();
        let from_pos = from_room.sample_edge(&dir, &mut rng).unwrap();
        let to_pos = to_room.sample_edge(&dir.invert(), &mut rng).unwrap();

        let wall = from.area.wall(&dir);

        let tile = Tile::Passage;
        for pos in get_poses_align(from_pos, to_pos, dir, wall) {
            map.tiles[pos.y as usize][pos.x as usize] = tile.clone();
        }
    }
}

pub struct DungeonGenerator;

impl Generator<Dungeon> for DungeonGenerator {
    fn generate(&mut self, map: &mut Map) -> Dungeon {
        let mut rng = rand::thread_rng();
        let floor = Rect {
            pos: (0, 0).into(),
            size: map.size,
        };
        let min_area_size = 7;
        let rects = devide_rec(&mut rng, floor, min_area_size);
        let regions = rects
            .iter()
            .map(|r| Region::new(r.clone()))
            .collect::<Vec<Region>>();
        for region in regions.iter() {
            build_region(map, region);
        }
        for c in regions.iter().combinations(2) {
            let (f, t) = (&c[0], &c[1]);
            build_passage(map, f, t);
        }
        Dungeon {
            tiles: map.tiles.clone(),
            areas: regions,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::geo::point::Point;

    #[test]
    fn test_get_poses_align() {
        // S * . .
        // . * G .
        // . | . .
        let from = Point::new(0, 0);
        let to = Point::new(2, 1);
        let dir = super::Direction::Right;
        let poses = super::get_poses_align(from, to, dir, 1);
        assert_eq!(
            poses,
            vec![(0, 0), (2, 1), (1, 0), (1, 1)]
                .iter()
                .map(|p| Point::new(p.0, p.1))
                .collect::<HashSet<Point>>()
        );
    }
}
