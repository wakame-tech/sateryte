use super::actions::Action;
use bevy::{
    app::Events,
    prelude::{EventWriter, Res},
};
use bevy_crossterm::prelude::{KeyCode, KeyEvent};

pub fn input_keys(keys: Res<Events<KeyEvent>>, mut sender: EventWriter<Action>) {
    for event in keys.get_reader().iter(&*keys) {
        let action = match event {
            &KeyEvent {
                code: KeyCode::Char('l'),
                modifiers: _,
            } => Some(Action::Right),
            &KeyEvent {
                code: KeyCode::Char('h'),
                modifiers: _,
            } => Some(Action::Left),
            &KeyEvent {
                code: KeyCode::Char('k'),
                modifiers: _,
            } => Some(Action::Up),
            &KeyEvent {
                code: KeyCode::Char('j'),
                modifiers: _,
            } => Some(Action::Down),
            _ => None,
        };
        if let Some(action) = action {
            sender.send(action);
        }
    }
}
