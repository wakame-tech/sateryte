use bevy::prelude::*;
use bevy_crossterm::components::{Position, Sprite, SpriteBundle, StyleMap};
use rand::prelude::IteratorRandom;

use crate::{
    dungeon_world::dungeon::Dungeon, geo::direction::Direction,
    player::components::entity_bundle::PlayerBundle,
};

/// フロアを生成後, プレイヤーをスポーンさせる
pub fn world_spawn_player(
    mut commands: Commands,
    dungeon_query: Query<&Dungeon, Changed<Dungeon>>,
    mut sprites: ResMut<Assets<Sprite>>,
    mut stylemaps: ResMut<Assets<bevy_crossterm::components::StyleMap>>,
) {
    for dungeon in dungeon_query.iter() {
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
            position: Position::new(spawn_pos.x, spawn_pos.y, 2),
            stylemap: color,
            ..Default::default()
        };
        let player = PlayerBundle::new(sprite, spawn_pos, Direction::Down);

        commands.spawn_bundle(player);
    }
}
