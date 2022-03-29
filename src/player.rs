use bevy::prelude::*;
use bevy_crossterm::components::{Position, Sprite, SpriteBundle, StyleMap};
use rand::prelude::IteratorRandom;

use crate::{geo::point::Point, world::dungeon::Dungeon};

use super::input::actions::Action;
#[derive(Component)]
pub struct Player;

impl Player {}

pub fn spawn_player(
    mut commands: Commands,
    dungeon_query: Query<&Dungeon, Added<Dungeon>>,
    mut sprites: ResMut<Assets<Sprite>>,
    mut stylemaps: ResMut<Assets<bevy_crossterm::components::StyleMap>>,
) {
    if let Some(dungeon) = dungeon_query.iter().next() {
        dbg!(dungeon);
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

        commands
            .spawn_bundle(SpriteBundle {
                sprite: player,
                position: Position::with_xy(spawn_pos.x, spawn_pos.y),
                stylemap: color,
                ..Default::default()
            })
            .insert(Player);
    }
}

pub fn action_listener(
    mut reader: EventReader<Action>,
    dungeon_query: Query<&Dungeon>,
    mut player_query: Query<(&Player, &mut Position)>,
) {
    for action in reader.iter() {
        if let Some((_, mut player)) = player_query.iter_mut().next() {
            if let Some(dungeon) = dungeon_query.iter().next() {
                // dbg!(&dungeon.areas);
                match action {
                    Action::Right | Action::Left | Action::Up | Action::Down => {
                        let pos = Point::new(player.x, player.y);
                        let diff = match action {
                            Action::Right => Point::new(1, 0),
                            Action::Left => Point::new(-1, 0),
                            Action::Up => Point::new(0, -1),
                            Action::Down => Point::new(0, 1),
                        };
                        let new_pos = pos + diff;
                        if dungeon.is_movable(new_pos) {
                            player.x = new_pos.x;
                            player.y = new_pos.y;
                        }
                    }
                }
            }
        }
    }
}
