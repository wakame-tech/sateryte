use bevy::prelude::*;
use bevy_crossterm::components::{Color, Sprite, Style};

#[derive(Component, Debug, Clone)]
pub enum MapItem {
    Potion,
}

#[derive(Bundle)]
pub struct ItemBundle {
    pub item: MapItem,
    #[bundle]
    pub sprite: SpriteBundle,
}

impl ItemBundle {
    pub fn new(item_type: MapItem, sprite: SpriteBundle) -> Self {
        Self {
            item: item_type,
            sprite,
        }
    }
}
