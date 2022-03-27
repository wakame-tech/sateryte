use std::fmt::Display;

use rand::{prelude::ThreadRng, Rng};

use super::{point::Point, size::Size};

#[derive(Debug, Clone)]
pub struct Rect {
    pub pos: Point,
    pub size: Size,
}

impl Display for Rect {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "pos: ({}, {}), size: ({} x {})",
            self.pos.x, self.pos.y, self.size.w, self.size.h
        )
    }
}

impl Rect {
    pub fn is_edge(&self, pos: Point) -> bool {
        self.is_horizontal_edge(pos) || self.is_vertical_edge(pos)
    }

    pub fn is_vertical_edge(&self, pos: Point) -> bool {
        pos.x == self.pos.x || pos.x == self.pos.x + self.size.w as i32 - 1
    }

    pub fn is_horizontal_edge(&self, pos: Point) -> bool {
        pos.y == self.pos.y || pos.y == self.pos.y + self.size.h as i32 - 1
    }

    pub fn shrink(&self, amount: Size) -> Rect {
        assert!(amount.w * 2 < self.size.w && amount.h * 2 < self.size.h);
        Rect {
            pos: self.pos + amount.into(),
            size: self.size - (amount + amount).into(),
        }
    }

    pub fn devide_horizontal(&self, n: usize) -> Option<Vec<Rect>> {
        if self.size.w < n || self.size.w - n < 5 {
            return None;
        }
        println!("\n{}", self);
        dbg!(n);
        let left = Rect {
            pos: self.pos,
            size: (n, self.size.h).into(),
        };
        let right = Rect {
            pos: self.pos + (n as i32, 0).into(),
            size: self.size - (n, 0).into(),
        };
        println!("{}\n{}", &left, &right);
        Some(vec![left, right])
    }

    pub fn devide_vertical(&self, n: usize) -> Option<Vec<Rect>> {
        if self.size.h < n || self.size.h - n < 5 {
            return None;
        }
        println!("{}", self);
        dbg!(n);
        let top = Rect {
            pos: self.pos,
            size: (self.size.w, n).into(),
        };
        let bottom = Rect {
            pos: self.pos + (0, n as i32).into(),
            size: self.size - (0, n).into(),
        };
        println!("{}\n{}", &top, &bottom);
        Some(vec![top, bottom])
    }

    pub fn sample(&self, rng: &mut ThreadRng) -> Size {
        dbg!(&self.size);
        let x = self.pos.x as usize + rng.gen_range(0..self.size.w);
        let y = self.pos.y as usize + rng.gen_range(0..self.size.h);
        (x, y).into()
    }
}
