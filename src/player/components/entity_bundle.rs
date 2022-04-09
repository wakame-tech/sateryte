use bevy::prelude::*;
use bevy_crossterm::components::SpriteBundle;

use super::status::{Exp, Hp, Level};
use crate::geo::{direction::Direction, point::Point};

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
    /// 座標
    /// NOTE:
    /// この座標を変更しても画面には反映されない
    /// sprite::Position を変更すると画面にも反映される
    pub position: Point,
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
    pub fn new(sprite: SpriteBundle, position: Point, direction: Direction) -> Self {
        Self {
            tag: IsPlayer,
            position,
            direction,
            level: Level(1),
            exp: Exp { value: 0, next: 10 },
            hp: Hp { value: 10, max: 10 },
            flags: Flags { is_dash: false },
            sprite,
        }
    }
}
