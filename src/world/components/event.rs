use bevy::prelude::*;

use crate::geo::{point::Point, size::Size};

use super::map_item::MapItem;

/// フロアが生成される時に呼ばれるイベント
#[derive(Debug, Component)]
pub struct WorldGenerateEvent {
    pub world_size: Size,
    pub world_name: String,
    pub floor: u16,
}

/// アイテムをスポーンさせる時に呼ばれるイベント
#[derive(Debug, Component)]
pub struct ItemSpawnEvent {
    pub item: MapItem,
    pub pos: Point,
}
