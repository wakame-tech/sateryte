use bevy_crossterm::components::Sprite;

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
