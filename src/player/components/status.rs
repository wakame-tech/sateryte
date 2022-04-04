use bevy::prelude::*;

/// HP
#[derive(Component)]
pub struct Hp {
    pub value: u32,
    pub max: u32,
}

/// レベル
#[derive(Component)]
pub struct Level(pub u32);

/// 経験値
#[derive(Component)]
pub struct Exp {
    pub value: u32,
    pub next: u32,
}
