use bevy::prelude::*;
use bevy_crossterm::components::SpriteBundle;

pub mod action_listener;
pub mod actions;
pub mod spawn;

#[derive(Component)]
pub struct Player;

#[derive(Bundle)]
pub struct PlayerBundle {
    pub tag: Player,
    #[bundle]
    pub sprite: SpriteBundle,
}
