use bevy::prelude::*;
use bevy_crossterm::components::{Position, Sprite, SpriteBundle, StyleMap};

use crate::world::{
    components::{
        event::{WorldGenerateEvent, WorldGeneratedEvent},
        map::Map,
        tile::tile_style,
    },
    dungeon_world::{
        dungeon::Dungeon,
        generator::{DungeonGenerator, Generator},
    },
};

/// フロアを生成する
pub fn spawn_floor(mut commands: Commands, mut events: EventReader<WorldGenerateEvent>) {
    for event in events.iter() {
        dbg!(&event);
        let mut map = Map::new(event.world_size);
        let mut generator = DungeonGenerator;
        let dungeon = generator.generate(&mut map);
        commands.spawn().insert(dungeon);
    }
}

/// フロアを描画する
pub fn listen_world_generated(
    mut commands: Commands,
    query: Query<&Dungeon, Changed<Dungeon>>,
    mut sprites: ResMut<Assets<Sprite>>,
    mut stylemaps: ResMut<Assets<StyleMap>>,
    mut writer: EventWriter<WorldGeneratedEvent>,
) {
    for dungeon in query.iter() {
        println!("changed dungeon");
        for (y, row) in dungeon.tiles.iter().enumerate() {
            for (x, tile) in row.iter().enumerate() {
                let stylemap = stylemaps.add(StyleMap::new(tile_style(tile), vec![]));
                let tile = sprites.add(tile.clone().into());
                commands.spawn_bundle(SpriteBundle {
                    sprite: tile,
                    position: Position::with_xy(x as i32, y as i32),
                    stylemap,
                    ..Default::default()
                });
            }
        }
        writer.send(WorldGeneratedEvent);
    }
}
