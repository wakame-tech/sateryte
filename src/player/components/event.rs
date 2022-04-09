use bevy::prelude::*;

use crate::geo::point::Point;

use super::action::Action;

/// プレイヤーがスポーンした時に発行される
#[derive(Debug, Component)]
pub struct PlayerSpawnedEvent;

/// プレイヤーが行動する毎に発行される
#[derive(Debug, Component)]
pub struct PlayerActedEvent {
    pub action: Action,
    pub pos: Point,
}
