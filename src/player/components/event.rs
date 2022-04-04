use bevy::prelude::*;

/// プレイヤーがスポーンした時に発行される
#[derive(Debug, Component)]
pub struct PlayerSpawnedEvent;

/// プレイヤーが1行動する毎に発行される
#[derive(Debug, Component)]
pub struct PlayerMoveEvent;
