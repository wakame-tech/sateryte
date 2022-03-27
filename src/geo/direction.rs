use super::point::Point;

#[derive(Debug, Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
}

impl Into<Point> for Direction {
    fn into(self) -> Point {
        match self {
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
}
