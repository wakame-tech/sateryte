use bevy::prelude::*;
use bevy_crossterm::components::SpriteBundle;

use super::status::{Exp, Hp, Level};
use crate::geo::direction::Direction;

/// プレイヤータグ
#[derive(Debug, Component)]
pub struct IsPlayer;

/// フラグ
#[derive(Debug, Component)]
pub struct Flags {
    /// ダッシュ中か
    pub is_dash: bool,
}

/// プレイヤー
#[derive(Bundle)]
pub struct PlayerBundle {
    pub tag: IsPlayer,
    /// 方向
    pub direction: Direction,
    pub level: Level,
    pub exp: Exp,
    pub hp: Hp,
    pub flags: Flags,
    #[bundle]
    pub sprite: SpriteBundle,
}

impl PlayerBundle {
    pub fn new(sprite: SpriteBundle) -> Self {
        Self {
            tag: IsPlayer,
            direction: Direction::Down,
            level: Level(1),
            exp: Exp { value: 0, next: 10 },
            hp: Hp { value: 10, max: 10 },
            flags: Flags { is_dash: false },
            sprite,
        }
    }
}
