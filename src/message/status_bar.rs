use std::collections::HashMap;

use bevy::{asset::HandleId, prelude::*};
use bevy_crossterm::components::{Color, Colors, Position, Sprite, SpriteBundle, StyleMap};
use terminal_size::{terminal_size, Width};

#[derive(Default, Component)]
pub struct StatusBar {
    pub props: HashMap<String, String>,
    pub handle: Option<HandleId>,
}

impl StatusBar {
    pub fn to_sprite(&self) -> Sprite {
        let (Width(width), _) = terminal_size().unwrap();
        let content = self
            .props
            .iter()
            .map(|(key, value)| format!("{}: {}", key, value))
            .collect::<Vec<String>>()
            .join(" ");
        let content = format!("{:width$}", content, width = width as usize);
        Sprite::new(content)
    }
}

#[derive(Component, Clone)]
pub struct StatusBarUpdateEvent {
    pub key: String,
    pub value: String,
}

impl StatusBarUpdateEvent {
    pub fn new(key: &str, value: &str) -> Self {
        Self {
            key: key.to_string(),
            value: value.to_string(),
        }
    }
}

pub fn status_bar_listener(
    mut commands: Commands,
    mut status_bar: ResMut<StatusBar>,
    mut reader: EventReader<StatusBarUpdateEvent>,
    mut sprites: ResMut<Assets<Sprite>>,
    mut stylemaps: ResMut<Assets<StyleMap>>,
) {
    for event in reader.iter() {
        status_bar
            .props
            .insert(event.key.clone(), event.value.clone());
        let new_sprite = status_bar.to_sprite();

        // handleがあれば更新, なければ作成
        // TODO: Resourceで代替できないか
        status_bar.handle = if let Some(handle) = status_bar.handle {
            sprites.set_untracked(handle, new_sprite);
            Some(handle)
        } else {
            let sprite = sprites.add(new_sprite);
            let handle: HandleId = sprite.clone().into();
            let color = stylemaps.add(StyleMap::with_colors(Colors::new(
                Color::White,
                Color::Cyan,
            )));
            let bundle = SpriteBundle {
                sprite,
                position: Position::with_xy(0, 28),
                stylemap: color,
                ..Default::default()
            };
            commands.spawn_bundle(bundle);
            Some(handle)
        }
    }
}
