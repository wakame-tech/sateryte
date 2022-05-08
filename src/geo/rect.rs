use std::fmt::Display;

use rand::{prelude::ThreadRng, Rng};

use super::{direction::Direction, point::Point, size::Size};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rect {
    pub pos: Point,
    pub size: Size,
}

impl Display for Rect {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "pos: {}, size: {}", self.pos, self.size)
    }
}

impl Rect {
    pub fn new(x: i32, y: i32, w: usize, h: usize) -> Self {
        Self {
            pos: Point::new(x, y),
            size: Size::new(w, h),
        }
    }

    pub fn from_tuple(tup: (i32, i32, usize, usize)) -> Self {
        Self {
            pos: Point::new(tup.0, tup.1),
            size: Size::new(tup.2, tup.3),
        }
    }

    pub fn as_tuple(&self) -> (i32, i32, usize, usize) {
        (self.pos.x, self.pos.y, self.size.w, self.size.h)
    }

    pub fn is_edge(&self, pos: Point) -> bool {
        self.is_horizontal_edge(pos) || self.is_vertical_edge(pos)
    }

    pub fn is_vertical_edge(&self, pos: Point) -> bool {
        pos.x == self.pos.x || pos.x == self.pos.x + self.size.w as i32 - 1
    }

    pub fn is_horizontal_edge(&self, pos: Point) -> bool {
        pos.y == self.pos.y || pos.y == self.pos.y + self.size.h as i32 - 1
    }

    pub fn shrink(&self, x: i32, y: i32) -> Option<Rect> {
        let amount = Point::new(x, y);
        if amount.x * 2 < self.size.w as i32 && amount.y * 2 < self.size.h as i32 {
            let pad = amount.times(2);
            Some(Rect {
                pos: self.pos + amount,
                size: self.size - Size::new(pad.x as usize, pad.y as usize),
            })
        } else {
            None
        }
    }

    pub fn contain(&self, pos: &Point) -> bool {
        pos.x >= self.pos.x
            && pos.x < self.pos.x + self.size.w as i32
            && pos.y >= self.pos.y
            && pos.y < self.pos.y + self.size.h as i32
    }

    /// get faced direction
    pub fn intercect(&self, other: &Rect) -> Option<Direction> {
        let res = if self.pos.x == other.pos.x + other.size.w as i32 {
            Some(Direction::Left)
        } else if self.pos.x + self.size.w as i32 == other.pos.x {
            Some(Direction::Right)
        } else if self.pos.y == other.pos.y + other.size.h as i32 {
            Some(Direction::Up)
        } else if self.pos.y + self.size.h as i32 == other.pos.y {
            Some(Direction::Down)
        } else {
            None
        };
        println!("{} vs {} => {:?}", self, other, res);
        res
    }

    pub fn devide_horizontal(&self, n: usize) -> Option<Vec<Rect>> {
        if self.size.w < n || self.size.w - n < 5 {
            return None;
        }
        let left = Rect {
            pos: self.pos,
            size: (n, self.size.h).into(),
        };
        let right = Rect {
            pos: self.pos + (n as i32, 0).into(),
            size: self.size - (n, 0).into(),
        };
        Some(vec![left, right])
    }

    pub fn devide_vertical(&self, n: usize) -> Option<Vec<Rect>> {
        if self.size.h < n || self.size.h - n < 5 {
            return None;
        }
        let top = Rect {
            pos: self.pos,
            size: (self.size.w, n).into(),
        };
        let bottom = Rect {
            pos: self.pos + (0, n as i32).into(),
            size: self.size - (0, n).into(),
        };
        Some(vec![top, bottom])
    }

    pub fn sample(&self, rng: &mut ThreadRng) -> Size {
        let x = self.pos.x as usize + rng.gen_range(0..self.size.w);
        let y = self.pos.y as usize + rng.gen_range(0..self.size.h);
        (x, y).into()
    }

    /// choose a pos on the edge of the rect but not on the corners
    pub fn sample_edge(&self, dir: &Direction, rng: &mut ThreadRng) -> Option<Point> {
        let sx = self.pos.x + 1;
        let sy = self.pos.y + 1;
        let ex = self.pos.x + self.size.w as i32 - 1;
        let ey = self.pos.y + self.size.h as i32 - 1;
        let rx = rng.gen_range(sx..ex);
        let ry = rng.gen_range(sy..ey);
        match dir {
            Direction::Up => Some(Point::new(rx, self.pos.y)),
            Direction::Down => Some(Point::new(rx, self.pos.y + self.size.h as i32 - 1)),
            Direction::Left => Some(Point::new(self.pos.x, ry)),
            Direction::Right => Some(Point::new(self.pos.x + self.size.w as i32 - 1, ry)),
            _ => None,
        }
    }

    /// returns edges position
    pub fn wall(&self, dir: &Direction) -> i32 {
        match dir {
            Direction::Left => self.pos.x,
            Direction::Right => self.pos.x + self.size.w as i32 - 1,
            Direction::Up => self.pos.y,
            Direction::Down => self.pos.y + self.size.h as i32 - 1,
            _ => panic!("invalid direction"),
        }
    }

    pub fn tl(&self) -> Point {
        self.pos
    }

    pub fn br(&self) -> Point {
        self.pos + self.size.into()
    }

    pub fn tr(&self) -> Point {
        self.pos + (self.size.w as i32, 0).into()
    }

    pub fn bl(&self) -> Point {
        self.pos + (0, self.size.h as i32).into()
    }

    pub fn center(&self) -> Point {
        self.pos + Size::new(self.size.w / 2, self.size.h / 2).into()
    }
}
