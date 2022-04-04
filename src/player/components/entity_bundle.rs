use bevy::prelude::*;
use bevy_crossterm::components::SpriteBundle;

use super::status::{Exp, Hp, Level};

/// プレイヤータグ
#[derive(Debug, Component)]
pub struct IsPlayer;

#[derive(Bundle)]
pub struct EntityBundle {
    pub tag: IsPlayer,
    pub level: Level,
    pub exp: Exp,
    pub hp: Hp,
    #[bundle]
    pub sprite: SpriteBundle,
}

impl EntityBundle {
    pub fn new(sprite: SpriteBundle) -> Self {
        Self {
            tag: IsPlayer,
            level: Level(1),
            exp: Exp { value: 0, next: 10 },
            hp: Hp { value: 10, max: 10 },
            sprite,
        }
    }
}
