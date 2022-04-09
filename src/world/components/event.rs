use bevy::prelude::*;

use crate::{
    dungeon_world::dungeon::Dungeon,
    geo::{point::Point, size::Size},
};

use super::map_item::MapItem;

/// フロアが生成される時に呼ばれるイベント
#[derive(Debug, Component)]
pub struct FloorGenerateEvent {
    /// ダンジョン名
    pub dungeon_name: String,
    /// 階層
    pub floor: u16,
    /// サイズ
    pub map_size: Size,
}

/// フロアが生成された時に呼ばれるイベント
#[derive(Debug, Component)]
pub struct FloorGeneratedEvent {
    pub dungeon: Dungeon,
}

/// アイテムをスポーンさせる時に呼ばれるイベント
#[derive(Debug, Component)]
pub struct ItemSpawnEvent {
    pub item: MapItem,
    pub pos: Point,
}
