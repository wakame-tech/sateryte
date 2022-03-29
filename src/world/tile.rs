use bevy_crossterm::components::{Color, Sprite};

#[derive(Debug, Clone)]

pub enum Tile {
    Wall,
    WallV,
    WallH,
    Region,
    Floor,
}

impl Tile {
    pub fn is_through(&self) -> bool {
        match self {
            Tile::Wall | Tile::WallV | Tile::WallH => false,
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
            Tile::Region => Sprite::new("+"),
            Tile::Floor => Sprite::new(" "),
        }
    }
}

pub fn tile_color(tile: &Tile) -> Color {
    match tile {
        Tile::Region => Color::Rgb {
            r: 128u8,
            g: 0u8,
            b: 0u8,
        },
        Tile::Wall => Color::Rgb {
            r: 128u8,
            g: 128u8,
            b: 128u8,
        },
        Tile::WallH | Tile::WallV => Color::Green,
        _ => Color::Black,
    }
}
