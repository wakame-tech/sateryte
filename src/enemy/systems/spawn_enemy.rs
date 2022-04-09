use bevy::prelude::*;
use rand::prelude::IteratorRandom;

use crate::{dungeon_world::dungeon::Dungeon, enemy::components::EnemyBundle};
use bevy_crossterm::components::{Position, Sprite, SpriteBundle, StyleMap};

pub fn world_spawn_enemy(
    mut commands: Commands,
    dungeon_query: Query<&Dungeon, Added<Dungeon>>,
    mut sprites: ResMut<Assets<Sprite>>,
    mut stylemaps: ResMut<Assets<StyleMap>>,
) {
    for dungeon in dungeon_query.iter() {
        let enemy = sprites.add(Sprite::new("A"));
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
            sprite: enemy,
            position: Position::new(spawn_pos.x, spawn_pos.y, 3),
            stylemap: color,
            ..Default::default()
        };
        let enemy = EnemyBundle::new(sprite);

        commands.spawn_bundle(enemy);
    }
}
