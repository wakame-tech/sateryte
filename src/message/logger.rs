use bevy::prelude::*;
use bevy_crossterm::components::{Color, Colors, Position, Sprite, SpriteBundle, StyleMap};
use terminal_size::{terminal_size, Height, Width};

#[derive(Component)]
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
        let (Width(w), Height(h)) = terminal_size().unwrap();
        let width = w - 80;
        let content = logger
            .messages
            .iter()
            .enumerate()
            .rev()
            .take(h.into())
            .map(|(i, mes)| {
                format!(
                    "[{}] {:width$}",
                    i,
                    mes.text.as_str(),
                    width = width as usize
                )
            })
            .collect::<Vec<String>>()
            .join("\n");
        let sprite = sprites.add(Sprite::new(content));
        let sprite = SpriteBundle {
            sprite,
            position: Position::with_xy(80, 0),
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
