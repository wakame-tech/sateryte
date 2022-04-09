use bevy::prelude::*;
use bevy_crossterm::components::{Position, Sprite, SpriteBundle, StyleMap};

use crate::{
    dungeon_world::generator::{DungeonGenerator, Generator},
    message::components::status_bar::StatusBarUpdateEvent,
    world::components::{event::WorldGenerateEvent, map::Map},
};

/// フロアを生成する
pub fn spawn_floor(
    mut commands: Commands,
    mut events: EventReader<WorldGenerateEvent>,
    mut writer: EventWriter<StatusBarUpdateEvent>,
) {
    for event in events.iter() {
        let mut map = Map::new(event.world_size);
        let mut generator = DungeonGenerator;
        let dungeon = generator.generate(&mut map);
        commands.spawn().insert(dungeon);

        let event = StatusBarUpdateEvent::new(
            "floor",
            format!("{} {}F", event.world_name, event.floor).as_str(),
        );
        writer.send(event);
    }
}
