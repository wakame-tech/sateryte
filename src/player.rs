use bevy::prelude::*;
use bevy_crossterm::components::{Position, Sprite, SpriteBundle, Style, StyleMap};
use crossterm::style;
use rand::prelude::IteratorRandom;

use crate::{geo::point::Point, message::Message, world::dungeon::Dungeon};

use super::input::actions::Action;

#[derive(Component)]
pub struct Player;

#[derive(Bundle)]
pub struct PlayerBundle {
    pub tag: Player,
    #[bundle]
    pub sprite: SpriteBundle,
}

pub fn spawn_player(
    mut commands: Commands,
    dungeon_query: Query<&Dungeon, Added<Dungeon>>,
    mut message_writer: EventWriter<Message>,
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
            position: Position::with_xy(spawn_pos.x, spawn_pos.y),
            stylemap: color,
            ..Default::default()
        };

        commands.spawn_bundle(PlayerBundle {
            tag: Player,
            sprite,
        });

        let text = style::style(format!("[spawn] ({}, {})", spawn_pos.x, spawn_pos.y))
            .with(style::Color::Yellow)
            .to_string();
        message_writer.send(Message { text });
    }
}

pub fn action_listener(
    mut reader: EventReader<Action>,
    dungeon_query: Query<&Dungeon>,
    mut player_query: Query<(&Player, &mut Position)>,
    mut message_writer: EventWriter<Message>,
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
                            let text =
                                style::style(format!("[move] ({}, {})", new_pos.x, new_pos.y))
                                    .with(style::Color::Yellow)
                                    .to_string();
                            message_writer.send(Message { text });
                        }
                    }
                }
            }
        }
    }
}
