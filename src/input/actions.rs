use bevy::prelude::Component;

#[derive(Debug, Component)]
pub enum Action {
    Right,
    // RightToWall,
    Left,
    // LeftToWall,
    Up,
    // UpToWall,
    Down,
    // DownToWall,
    // UpRight,
    // UpRightToWall,
    // UpLeft,
    // UpLeftToWall,
    // DownRight,
    // DownRightToWall,
    // DownLeft,
    // DownLeftToWall,
}
