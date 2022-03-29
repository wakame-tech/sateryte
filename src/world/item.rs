use bevy::prelude::*;
use bevy_crossterm::components::{Color, Position, Sprite, SpriteBundle, Style, StyleMap};

use super::dungeon::Dungeon;

#[derive(Component, Debug, Clone)]
pub enum MapItem {
    Potion,
}

pub fn startup_map_item(
    mut commands: Commands,
    dungeon_query: Query<&Dungeon>,
    mut sprites: ResMut<Assets<Sprite>>,
    mut stylemaps: ResMut<Assets<bevy_crossterm::components::StyleMap>>,
) {
    let mut rng = rand::thread_rng();
    let dungeon = dungeon_query.single();

    let color = stylemaps.add(StyleMap::default());
    for region in &dungeon.areas {
        // spawn items
        for _ in 0..3 {
            if let Some(pos) = region.random_floor(&mut rng) {
                let item: Sprite = MapItem::Potion.into();
                commands.spawn_bundle(SpriteBundle {
                    sprite: sprites.add(item),
                    position: Position::with_xy(pos.x, pos.y),
                    stylemap: color.clone(),
                    ..Default::default()
                });
            }
        }
    }
}

impl Into<Sprite> for MapItem {
    fn into(self) -> Sprite {
        match self {
            // Tile::Gold(_) => Sprite::new("&"),
            MapItem::Potion => Sprite::new("*"),
        }
    }
}

pub fn item_style(item: &MapItem) -> Style {
    match item {
        MapItem::Potion => Style::with_fg(Color::Yellow),
    }
}
