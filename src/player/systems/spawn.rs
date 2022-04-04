use bevy::prelude::*;
use bevy_crossterm::components::{Position, Sprite, SpriteBundle, StyleMap};
use crossterm::style;
use rand::prelude::IteratorRandom;

use crate::{
    player::components::entity_bundle::EntityBundle,
    world::{components::event::WorldGeneratedEvent, dungeon_world::dungeon::Dungeon},
};

/// フロアを生成後, プレイヤーをスポーンさせる
pub fn world_spawn_player(
    mut commands: Commands,
    mut reader: EventReader<WorldGeneratedEvent>,
    dungeon_query: Query<&Dungeon, Added<Dungeon>>,
    mut sprites: ResMut<Assets<Sprite>>,
    mut stylemaps: ResMut<Assets<bevy_crossterm::components::StyleMap>>,
) {
    for _ in reader.iter() {
        let dungeon = dungeon_query.single();
        let player = sprites.add(Sprite::new("@"));
        let color = stylemaps.add(StyleMap::default());

        let mut rng = rand::thread_rng();
        let spawn_pos = dungeon
            .areas
            .iter()
            .filter(|a| a.room.is_some())
            .choose(&mut rng)
            .unwrap()
            .random_floor(&mut rng)
            .unwrap();

        let sprite = SpriteBundle {
            sprite: player,
            position: Position::with_xy(spawn_pos.x, spawn_pos.y),
            stylemap: color,
            ..Default::default()
        };
        let player = EntityBundle::new(sprite);

        commands.spawn_bundle(player);
    }
}
