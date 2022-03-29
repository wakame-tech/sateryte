use std::collections::HashMap;

use bevy::prelude::*;
use bevy_crossterm::components::{Color, Colors, Position, Sprite, SpriteBundle, StyleMap};
use terminal_size::{terminal_size, Width};

#[derive(Component)]
pub struct StatusBar {
    pub map: HashMap<String, String>,
}

#[derive(Component, Clone)]
pub struct StatusBarUpdateEvent {
    pub key: String,
    pub value: String,
}

pub fn setup_status_bar(mut commands: Commands) {
    commands.spawn().insert(StatusBar {
        map: HashMap::new(),
    });
}

pub fn status_bar_listener(
    mut status_bar_query: Query<&mut StatusBar>,
    mut reader: EventReader<StatusBarUpdateEvent>,
) {
    for event in reader.iter() {
        let mut status_bar = status_bar_query.single_mut();
        status_bar
            .map
            .insert(event.key.clone(), event.value.clone());
    }
}

pub fn draw_status_bar(
    mut commands: Commands,
    status_bar_query: Query<&StatusBar, Changed<StatusBar>>,
    mut sprites: ResMut<Assets<Sprite>>,
    mut stylemaps: ResMut<Assets<StyleMap>>,
) {
    for status_bar in status_bar_query.iter() {
        let color = stylemaps.add(StyleMap::with_colors(Colors::new(
            Color::White,
            Color::Blue,
        )));
        let (Width(width), _) = terminal_size().unwrap();
        let content = status_bar
            .map
            .iter()
            .map(|(key, value)| format!("{}: {}", key, value))
            .collect::<Vec<String>>()
            .join(" ");
        let content = format!("{:width$}", content, width = width as usize);
        let sprite = sprites.add(Sprite::new(content));
        let sprite = SpriteBundle {
            sprite,
            position: Position::with_xy(0, 28),
            stylemap: color,
            ..Default::default()
        };
        commands.spawn_bundle(sprite);
    }
}
