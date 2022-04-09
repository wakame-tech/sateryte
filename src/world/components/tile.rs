use bevy_crossterm::components::{Color, Sprite, Style};

/// マップの地形
#[derive(Debug, Clone, PartialEq, Eq)]

pub enum Tile {
    /// 壁
    Wall,
    /// フロアと水平に隣接している壁
    WallV,
    /// フロアと垂直に隣接している壁
    WallH,
    /// デバッグ用, 任意の文字
    Debug(String),
    /// 通路
    Passage,
    /// フロア
    Floor,
    /// 階段
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
    let debug_color = Color::DarkYellow;
    let wall_color = Color::Rgb {
        r: 128u8,
        g: 128u8,
        b: 128u8,
    };
    let edge_wall_color = Color::Green;

    match tile {
        Tile::Debug(_) => Style::with_fg(debug_color),
        Tile::Wall => Style::with_fg(wall_color),
        Tile::WallH | Tile::WallV | Tile::Passage => Style::with_fg(edge_wall_color),
        _ => Style::default(),
    }
}
