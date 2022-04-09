use bevy::prelude::*;
use bevy_crossterm::components::{Position, Sprite, SpriteBundle, StyleMap};

use crate::{
    dungeon_world::dungeon::Dungeon, message::components::status_bar::StatusBarUpdateEvent,
    world::components::tile::tile_style,
};

/// デバッグ用, タイルコンポーネントの数を数える
pub fn profile_tile_count(
    mut commands: Commands,
    dungeon: Option<Res<Dungeon>>,
    tile_query: Query<&Position>,
    mut sprites: ResMut<Assets<Sprite>>,
    mut stylemaps: ResMut<Assets<StyleMap>>,
    mut status_bar: EventWriter<StatusBarUpdateEvent>,
) {
    if let Some(ref dungeon) = dungeon {
        let mut count = 0;
        for _ in tile_query.iter() {
            count += 1;
        }
        let event = StatusBarUpdateEvent::new("tiles", count.to_string().as_str());
        status_bar.send(event);

        if !dungeon.is_changed() {
            return;
        }
        // tile の再描画
        for (y, row) in dungeon.tiles.iter().enumerate() {
            for (x, tile) in row.iter().enumerate() {
                let stylemap = stylemaps.add(StyleMap::new(tile_style(tile), vec![]));
                let tile = sprites.add(tile.clone().into());
                commands.spawn_bundle(SpriteBundle {
                    sprite: tile,
                    position: Position::new(x as i32, y as i32, 0),
                    stylemap,
                    ..Default::default()
                });
            }
        }
    }
}
