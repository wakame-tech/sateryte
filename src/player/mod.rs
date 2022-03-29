use bevy::prelude::*;
use bevy_crossterm::components::SpriteBundle;

pub mod action_listener;
pub mod actions;
pub mod spawn;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Hp {
    pub value: u32,
    pub max: u32,
}

#[derive(Component)]
pub struct Level(pub u32);

#[derive(Component)]
pub struct Exp {
    pub value: u32,
    pub next: u32,
}

#[derive(Bundle)]
pub struct PlayerBundle {
    pub tag: Player,
    pub level: Level,
    pub exp: Exp,
    pub hp: Hp,
    #[bundle]
    pub sprite: SpriteBundle,
}

impl PlayerBundle {
    fn new(sprite: SpriteBundle) -> Self {
        Self {
            tag: Player,
            level: Level(1),
            exp: Exp { value: 0, next: 10 },
            hp: Hp { value: 10, max: 10 },
            sprite,
        }
    }
}
