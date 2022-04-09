use bevy::prelude::*;
use bevy_crossterm::components::{Color, Position, Sprite, SpriteBundle, Style, StyleMap};

use crate::{
    dungeon_world::dungeon::Dungeon, message::logger::LogEvent,
    world::components::event::ItemSpawnEvent,
};

#[derive(Component, Debug, Clone)]
pub enum MapItem {
    Potion,
}

pub fn spawn_items(
    dungeon_query: Query<&Dungeon, Added<Dungeon>>,
    mut writer: EventWriter<ItemSpawnEvent>,
) {
    for dungeon in dungeon_query.iter() {
        println!("spawn_items");
        let mut rng = rand::thread_rng();
        for region in &dungeon.areas {
            // spawn items
            for _ in 0..3 {
                if let Some(pos) = region.random_floor(&mut rng) {
                    let event = ItemSpawnEvent {
                        pos,
                        item: MapItem::Potion,
                    };
                    writer.send(event);
                }
            }
        }
    }
}

pub fn item_char(item: &MapItem) -> char {
    match item {
        MapItem::Potion => '*',
    }
}

pub fn item_style(item: &MapItem) -> Style {
    match item {
        MapItem::Potion => Style::with_fg(Color::Yellow),
    }
}

/// アイテムを描画する
pub fn render_item(
    mut commands: Commands,
    mut sprites: ResMut<Assets<Sprite>>,
    mut stylemaps: ResMut<Assets<bevy_crossterm::components::StyleMap>>,
    reader: &mut EventReader<ItemSpawnEvent>,
    mut logger: EventWriter<LogEvent>,
) {
    for event in reader.iter() {
        let char = item_char(&event.item);
        let style = item_style(&event.item);
        let sprite = sprites.add(Sprite::new(char));
        let stylemap = stylemaps.add(StyleMap::new(style, vec![]));
        commands.spawn_bundle(SpriteBundle {
            sprite,
            position: event.pos.clone().into(),
            stylemap,
            ..Default::default()
        });
        logger.send(LogEvent::info(
            format!("spawn item: {:?}", event.item).as_str(),
        ));
    }
}
