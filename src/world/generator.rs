use bevy::prelude::*;
use rand::{prelude::ThreadRng, Rng};

use crate::geo::rect::Rect;

use super::{map::Map, region::Region};

pub trait Generator {
    fn generate(&mut self, map: &mut Map);
}

#[derive(Component)]
pub struct DefaultGenerator {
    pub areas: Vec<Region>,
}

impl DefaultGenerator {
    pub fn new() -> Self {
        Self { areas: vec![] }
    }
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

impl Generator for DefaultGenerator {
    fn generate(&mut self, map: &mut Map) {
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
            region.draw(map);
        }
        self.areas = regions;
    }
}
