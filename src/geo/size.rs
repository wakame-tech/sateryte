use std::{
    fmt::Display,
    ops::{Add, AddAssign, Sub, SubAssign},
};

use super::{point::Point, rect::Rect};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Size {
    pub w: usize,
    pub h: usize,
}

impl Display for Size {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} x {}", self.w, self.h)
    }
}

impl Size {
    pub fn new(w: usize, h: usize) -> Self {
        Self { w, h }
    }

    pub fn at(self, x: i32, y: i32) -> Rect {
        Rect::new(x, y, self.w, self.h)
    }
}

impl Into<Size> for (usize, usize) {
    fn into(self) -> Size {
        Size {
            w: self.0,
            h: self.1,
        }
    }
}

impl Into<Point> for Size {
    fn into(self) -> Point {
        Point {
            x: self.w as i32,
            y: self.h as i32,
        }
    }
}

impl Add for Size {
    type Output = Size;

    fn add(self, other: Size) -> Size {
        Size {
            w: self.w + other.w,
            h: self.h + other.h,
        }
    }
}

impl AddAssign for Size {
    fn add_assign(&mut self, other: Size) {
        self.w += other.w;
        self.h += other.h;
    }
}

impl Sub for Size {
    type Output = Size;

    fn sub(self, other: Size) -> Size {
        Size {
            w: self.w - other.w,
            h: self.h - other.h,
        }
    }
}

impl SubAssign for Size {
    fn sub_assign(&mut self, other: Size) {
        self.w -= other.w;
        self.h -= other.h;
    }
}
