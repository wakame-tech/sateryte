use bevy::prelude::*;
use bevy_crossterm::components::{Color, Position, Sprite, SpriteBundle, StyleMap};
use log;

use crate::message::components::logger::{LogEvent, Logger, LoggerOptions};

pub fn logger_listener(
    mut commands: Commands,
    mut logger: ResMut<Logger>,
    mut reader: EventReader<LogEvent>,
    mut sprites: ResMut<Assets<Sprite>>,
    mut stylemaps: ResMut<Assets<StyleMap>>,
    logger_options: Res<LoggerOptions>,
) {
    for message in reader.iter() {
        logger.messages.push(LogEvent {
            text: message.text.clone(),
        });
        log::info!("[game logger] {}", message.text);

        // let color = stylemaps.add(StyleMap::with_fg(Color::White));
        // let content = format!(
        //     "[{}] {:width$}",
        //     logger.messages.len() - 1,
        //     message.text.as_str(),
        //     width = logger_options.area.size.w,
        // );
        // let sprite = sprites.add(Sprite::new(content));
        // let sprite = SpriteBundle {
        //     sprite,
        //     position: logger_options.area.pos.into(),
        //     stylemap: color,
        //     ..Default::default()
        // };
        // commands.spawn_bundle(sprite);
    }
}
