use bevy::prelude::*;
use bevy_crossterm::components::{Position, Sprite, SpriteBundle, StyleMap};

use crate::{dungeon_world::dungeon::Dungeon, world::components::tile::tile_style};

/// [Dungeon] が変化する度にフロアを描画する
pub fn render_tiles(
    mut commands: Commands,
    query: Query<&Dungeon, Changed<Dungeon>>,
    mut sprites: ResMut<Assets<Sprite>>,
    mut stylemaps: ResMut<Assets<StyleMap>>,
) {
    for dungeon in query.iter() {
        // tile の描画
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
