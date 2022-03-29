use bevy::prelude::*;
use bevy_crossterm::components::{Color, Colors, Position, Sprite, SpriteBundle, StyleMap};
use terminal_size::{terminal_size, Width};

#[derive(Component)]
pub struct Logger {
    pub messages: Vec<LogEvent>,
}

#[derive(Component, Clone)]
pub struct LogEvent {
    pub text: String,
}
pub fn setup_logger(mut commands: Commands) {
    commands.spawn().insert(Logger {
        messages: Vec::new(),
    });
}

pub fn draw_logger(
    mut commands: Commands,
    logger_query: Query<&Logger, Changed<Logger>>,
    mut sprites: ResMut<Assets<Sprite>>,
    mut stylemaps: ResMut<Assets<StyleMap>>,
) {
    for logger in logger_query.iter() {
        let color = stylemaps.add(StyleMap::with_colors(Colors::new(
            Color::Black,
            Color::White,
        )));
        let (Width(width), _) = terminal_size().unwrap();
        let content = logger
            .messages
            .iter()
            .rev()
            .take(3)
            .map(|mes| format!("{:width$}", mes.text.as_str(), width = width as usize))
            .collect::<Vec<String>>()
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

pub fn logger_listener(mut logger_query: Query<&mut Logger>, mut reader: EventReader<LogEvent>) {
    for message in reader.iter() {
        let mut mes = logger_query.single_mut();
        mes.messages.push(LogEvent {
            text: message.text.clone(),
        });
    }
}
