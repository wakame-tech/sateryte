use bevy::prelude::*;
use bevy_crossterm::components::{Position, Sprite, SpriteBundle, StyleMap};
use crossterm::style;
use rand::prelude::IteratorRandom;

use crate::{message::Message, world::dungeon::Dungeon};

use super::{Player, PlayerBundle};

pub fn world_spawn_player(
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
