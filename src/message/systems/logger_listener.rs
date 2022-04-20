use bevy::prelude::*;
use log;

use crate::message::components::logger::{LogEvent, Logger};

pub fn logger_listener(mut logger: ResMut<Logger>, mut reader: EventReader<LogEvent>) {
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
