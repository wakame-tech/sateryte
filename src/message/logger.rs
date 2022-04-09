use std::{fs::OpenOptions, io::Write};

use bevy::prelude::*;
use bevy_crossterm::components::{Color, Colors, Position, Sprite, SpriteBundle, StyleMap};

#[derive(Component, Default)]
pub struct Logger {
    pub messages: Vec<LogEvent>,
}

#[derive(Component, Clone)]
pub struct LogEvent {
    pub text: String,
}

impl LogEvent {
    pub fn info(text: &str) -> Self {
        Self {
            text: format!("[info] {}", text),
        }
    }
}

// デバッグ用
pub fn dump_log(message: String) {
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open("debug.log")
        .unwrap();

    let mut writer = std::io::BufWriter::new(file);
    writer.write(format!("{}\n", message).as_bytes()).unwrap();
}

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
        dump_log(message.text.clone());

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
