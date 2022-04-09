use bevy::prelude::*;

use bevy_crossterm::components::SpriteBundle;

#[derive(Debug, Component)]
pub struct Enemy;

#[derive(Bundle)]
pub struct EnemyBundle {
    pub tag: Enemy,
    #[bundle]
    pub sprite: SpriteBundle,
}

impl EnemyBundle {
    pub fn new(sprite: SpriteBundle) -> Self {
        Self { tag: Enemy, sprite }
    }
}

/// 敵の行動が完了した時に発行される
#[derive(Debug, Component)]
pub struct EnemyMovedEvent;
