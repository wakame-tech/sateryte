use bevy::prelude::*;
use bevy_crossterm::components::{Position, Sprite, SpriteBundle, StyleMap};

#[derive(Component)]
pub struct MessageWindow {
    pub messages: Vec<Message>,
}

#[derive(Component, Clone)]
pub struct Message {
    pub text: String,
}

pub fn setup_message_window(mut commands: Commands) {
    commands.spawn().insert(MessageWindow {
        messages: Vec::new(),
    });
}

pub fn draw_message_window(
    mut commands: Commands,
    messages: Query<&MessageWindow, Changed<MessageWindow>>,
    mut sprites: ResMut<Assets<Sprite>>,
    mut stylemaps: ResMut<Assets<bevy_crossterm::components::StyleMap>>,
) {
    for mes in messages.iter() {
        let color = stylemaps.add(StyleMap::default());
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

pub struct MessagePlugin;

impl Plugin for MessagePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<Message>()
            .add_startup_system(setup_message_window)
            .add_system(message_listener)
            .add_system(draw_message_window);
    }
}
