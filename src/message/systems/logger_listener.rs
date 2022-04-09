use bevy::prelude::*;
use bevy_crossterm::components::{Sprite, StyleMap};
use log;

use crate::message::components::logger::{LogEvent, Logger};

pub fn logger_listener(
    mut commands: Commands,
    mut logger: ResMut<Logger>,
    mut reader: EventReader<LogEvent>,
    mut sprites: ResMut<Assets<Sprite>>,
    mut stylemaps: ResMut<Assets<StyleMap>>,
) {
    for message in reader.iter() {
        logger.messages.push(LogEvent {
            text: message.text.clone(),
        });
        log::info!("[game logger] {}", message.text);

        // let color = stylemaps.add(StyleMap::with_colors(Colors::new(
        //     Color::Black,
        //     Color::White,
        // )));
        // let (Width(w), Height(h)) = terminal_size().unwrap();
        // let width = w - 80;
        // let content = format!(
        //     "[{}] {:width$}",
        //     logger.messages.len() - 1,
        //     message.text.as_str(),
        //     width = width as usize
        // );
        // let sprite = sprites.add(Sprite::new(content));
        // let sprite = SpriteBundle {
        //     sprite,
        //     position: Position::with_xy(80, (logger.messages.len() % (h as usize)) as i32),
        //     stylemap: color,
        //     ..Default::default()
        // };
        // commands.spawn_bundle(sprite);
    }
}
