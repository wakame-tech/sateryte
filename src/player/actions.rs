use crate::geo::direction::Direction;
use bevy::prelude::*;

/// represents [Player] action
/// action may consumes multiple turns
#[derive(Debug, Component)]
pub enum Action {
    Walk(Direction),
    WalkToWall(Direction),
}
