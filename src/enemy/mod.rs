use bevy::prelude::*;
use rand::prelude::{IteratorRandom, SliceRandom};

use crate::{
    geo::direction::Direction, geo::point::Point, player::components::event::PlayerMoveEvent,
    world::dungeon_world::dungeon::Dungeon,
};
use bevy_crossterm::components::{Position, Sprite, SpriteBundle, StyleMap};

#[derive(Debug, Component)]
pub struct Enemy;

#[derive(Bundle)]
pub struct EnemyBundle {
    pub tag: Enemy,
    #[bundle]
    pub sprite: SpriteBundle,
}

impl EnemyBundle {
    pub fn new(sprite: SpriteBundle) -> Self {
        Self { tag: Enemy, sprite }
    }
}

pub fn world_spawn_enemy(
    mut commands: Commands,
    dungeon_query: Query<&Dungeon, Added<Dungeon>>,
    mut sprites: ResMut<Assets<Sprite>>,
    mut stylemaps: ResMut<Assets<bevy_crossterm::components::StyleMap>>,
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
            position: Position::with_xy(spawn_pos.x, spawn_pos.y),
            stylemap: color,
            ..Default::default()
        };
        let enemy = EnemyBundle::new(sprite);

        commands.spawn_bundle(enemy);
    }
}

pub fn enemy_move(
    mut player_move_event: EventReader<PlayerMoveEvent>,
    dungeon_query: Query<&Dungeon>,
    mut enemy_query: Query<(&Enemy, &mut Position)>,
) {
    for _ in player_move_event.iter() {
        if let Some(dungeon) = dungeon_query.iter().next() {
            for (enemy, mut position) in enemy_query.iter_mut() {
                let mut rng = rand::thread_rng();
                let new_pos = dungeon.get_next_pos(
                    Point::new(position.x, position.y),
                    Direction::around_4().choose(&mut rng).unwrap(),
                );
                if new_pos != Point::new(position.x, position.y) {
                    position.x = new_pos.x;
                    position.y = new_pos.y;
                }
            }
        }
    }
}
