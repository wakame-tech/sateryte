use std::{
    cmp::{max, min},
    fmt::Display,
    ops::{Add, AddAssign, Index, IndexMut, Neg, Sub, SubAssign},
};

use bevy::prelude::Component;
use bevy_crossterm::components::Position;

use super::direction::Direction;

#[derive(Component, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl<T> Index<Point> for Vec<Vec<T>> {
    type Output = T;

    fn index(&self, index: Point) -> &Self::Output {
        &self[index.y as usize][index.x as usize]
    }
}

impl<T> IndexMut<Point> for Vec<Vec<T>> {
    fn index_mut(&mut self, index: Point) -> &mut Self::Output {
        &mut self[index.y as usize][index.x as usize]
    }
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

    pub fn from_dir(d: &Direction) -> Self {
        match d {
            Direction::Up => Point::new(0, -1),
            Direction::Down => Point::new(0, 1),
            Direction::Left => Point::new(-1, 0),
            Direction::Right => Point::new(1, 0),
            Direction::UpLeft => Point::new(-1, -1),
            Direction::UpRight => Point::new(1, -1),
            Direction::DownLeft => Point::new(-1, 1),
            Direction::DownRight => Point::new(1, 1),
        }
    }

    pub fn zero() -> Self {
        Self { x: 0, y: 0 }
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

    /// 周囲 [distance] マス
    pub fn around(&self, distance: i32) -> Vec<Point> {
        let mut points = Vec::new();
        for dy in -distance..=distance {
            for dx in -distance..=distance {
                if dx == 0 && dy == 0 {
                    continue;
                }
                let new_pos = *self + Point::new(dx, dy);
                points.push(new_pos);
            }
        }
        points
    }

    /// 周囲4方向
    pub fn around4(&self) -> Vec<Point> {
        let mut points = Vec::new();
        for d in Direction::around_4() {
            points.push(*self + Point::from_dir(&d));
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

impl Into<Position> for Point {
    fn into(self) -> Position {
        Position::with_xy(self.x, self.y)
    }
}

impl Neg for Point {
    type Output = Point;

    fn neg(self) -> Point {
        Point {
            x: -self.x,
            y: -self.y,
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
