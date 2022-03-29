use std::{
    cmp::{max, min},
    fmt::Display,
    ops::{Add, AddAssign, Sub, SubAssign},
};

use bevy_crossterm::components::Position;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn times(&self, k: i32) -> Self {
        Self {
            x: self.x * k,
            y: self.y * k,
        }
    }

    pub fn toward_h(&self, x: i32) -> Vec<Point> {
        let mut points = Vec::new();
        let s = min(self.x, x);
        let e = max(self.x, x);
        for x in s..=e {
            points.push(Point::new(x, self.y));
        }
        points
    }

    pub fn toward_v(&self, y: i32) -> Vec<Point> {
        let mut points = Vec::new();
        let s = min(self.y, y);
        let e = max(self.y, y);
        for y in s..=e {
            points.push(Point::new(self.x, y));
        }
        points
    }
}

impl Into<Point> for (i32, i32) {
    fn into(self) -> Point {
        Point {
            x: self.0,
            y: self.1,
        }
    }
}

impl Into<Point> for Position {
    fn into(self) -> Point {
        Point {
            x: self.x,
            y: self.y,
        }
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, other: Point) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, other: Point) -> Point {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl SubAssign for Point {
    fn sub_assign(&mut self, other: Point) {
        self.x -= other.x;
        self.y -= other.y;
    }
}
