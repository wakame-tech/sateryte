use bevy::prelude::*;
use bevy_crossterm::components::{Position, Sprite, SpriteBundle, StyleMap};

use crate::{
    dungeon_world::generator::{DungeonGenerator, Generator},
    message::components::status_bar::StatusBarUpdateEvent,
    world::components::{
        event::{FloorGenerateEvent, FloorGeneratedEvent},
        map::Floor,
        tile::tile_style,
    },
};

/// フロアを生成する
pub fn spawn_floor(
    mut commands: Commands,
    mut events: EventReader<FloorGenerateEvent>,
    mut floor_generated: EventWriter<FloorGeneratedEvent>,
    mut sprites: ResMut<Assets<Sprite>>,
    mut stylemaps: ResMut<Assets<StyleMap>>,
    mut status_bar: EventWriter<StatusBarUpdateEvent>,
) {
    for event in events.iter() {
        let mut map = Floor::new(event.map_size);
        let mut generator = DungeonGenerator;
        let dungeon = generator.generate(&mut map);
        commands.insert_resource(dungeon.clone());

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

        let status_bar_update = StatusBarUpdateEvent::new(
            "floor",
            format!("{} {}F", event.dungeon_name, event.floor).as_str(),
        );
        status_bar.send(status_bar_update);
        floor_generated.send(FloorGeneratedEvent { dungeon });
    }
}
