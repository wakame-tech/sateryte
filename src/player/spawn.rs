use bevy::prelude::*;
use bevy_crossterm::components::{Position, Sprite, SpriteBundle, StyleMap};
use crossterm::style;
use rand::prelude::IteratorRandom;

use crate::{
    message::{logger::LogEvent, status_bar::StatusBarUpdateEvent},
    world::dungeon::Dungeon,
};

use super::PlayerBundle;

pub fn world_spawn_player(
    mut commands: Commands,
    dungeon_query: Query<&Dungeon, Added<Dungeon>>,
    mut status_bar: EventWriter<StatusBarUpdateEvent>,
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
        let player = PlayerBundle::new(sprite);

        status_bar.send(StatusBarUpdateEvent {
            key: "lv".to_string(),
            value: player.level.0.to_string(),
        });
        status_bar.send(StatusBarUpdateEvent {
            key: "exp".to_string(),
            value: format!("{}/{}", player.exp.value, player.exp.next),
        });
        status_bar.send(StatusBarUpdateEvent {
            key: "hp".to_string(),
            value: format!("{}/{}", player.hp.value, player.hp.max),
        });

        commands.spawn_bundle(player);
    }
}
