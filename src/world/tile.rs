use bevy_crossterm::components::{Color, Sprite, Style};

#[derive(Debug, Clone, PartialEq, Eq)]

pub enum Tile {
    Wall,
    WallV,
    WallH,
    Debug(String),
    Passage,
    Floor,
    Stairs,
}

impl Tile {
    pub fn is_through(&self) -> bool {
        match self {
            Tile::Wall | Tile::WallV | Tile::WallH | Tile::Stairs => false,
            _ => true,
        }
    }
}

impl Into<Sprite> for Tile {
    fn into(self) -> Sprite {
        match self {
            Tile::Wall => Sprite::new("."),
            Tile::WallV => Sprite::new("|"),
            Tile::WallH => Sprite::new("-"),
            Tile::Passage => Sprite::new("#"),
            Tile::Floor => Sprite::new(" "),
            Tile::Stairs => Sprite::new("/"),
            Tile::Debug(s) => Sprite::new(s),
        }
    }
}

pub fn tile_style(tile: &Tile) -> Style {
    match tile {
        Tile::Debug(_) => Style::with_fg(Color::DarkYellow),
        Tile::Wall => Style::with_fg(Color::Rgb {
            r: 128u8,
            g: 128u8,
            b: 128u8,
        }),
        Tile::WallH | Tile::WallV | Tile::Passage => Style::with_fg(Color::Green),
        _ => Style::default(),
    }
}
