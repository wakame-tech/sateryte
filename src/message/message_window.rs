use bevy::prelude::*;
use bevy_crossterm::components::{Color, Colors, Position, Sprite, SpriteBundle, StyleMap};

use super::{Message, MessageWindow};

pub fn setup_message_window(mut commands: Commands) {
    commands.spawn().insert(MessageWindow {
        messages: Vec::new(),
    });
}

pub fn draw_message_window(
    mut commands: Commands,
    messages: Query<&MessageWindow, Changed<MessageWindow>>,
    mut sprites: ResMut<Assets<Sprite>>,
    mut stylemaps: ResMut<Assets<StyleMap>>,
) {
    for mes in messages.iter() {
        let color = stylemaps.add(StyleMap::with_colors(Colors::new(
            Color::Black,
            Color::White,
        )));
        let content = mes
            .messages
            .iter()
            .rev()
            .take(3)
            .map(|mes| mes.text.as_str())
            .collect::<Vec<&str>>()
            .join("\n");
        let sprite = sprites.add(Sprite::new(content));
        let sprite = SpriteBundle {
            sprite,
            position: Position::with_xy(0, 25),
            stylemap: color,
            ..Default::default()
        };
        commands.spawn_bundle(sprite);
    }
}

pub fn message_listener(mut messages: Query<&mut MessageWindow>, mut reader: EventReader<Message>) {
    for message in reader.iter() {
        let mut mes = messages.single_mut();
        mes.messages.push(Message {
            text: message.text.clone(),
        });
    }
}
