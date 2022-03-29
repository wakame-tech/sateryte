use crate::{geo::direction::Direction, player::actions::Action};
use bevy::{app::Events, prelude::*};
use bevy_crossterm::prelude::{KeyCode, KeyEvent};

macro_rules! key {
    ( $char: literal ) => {
        &KeyEvent {
            code: KeyCode::Char($char),
            modifiers: _,
        }
    };
}

/// convert input key to action
pub fn input_keys(keys: Res<Events<KeyEvent>>, mut sender: EventWriter<Action>) {
    for event in keys.get_reader().iter(&*keys) {
        let action = match event {
            key!('l') => Some(Action::Walk(Direction::Right)),
            key!('L') => Some(Action::WalkToWall(Direction::Right)),
            key!('h') => Some(Action::Walk(Direction::Left)),
            key!('H') => Some(Action::WalkToWall(Direction::Left)),
            key!('k') => Some(Action::Walk(Direction::Up)),
            key!('K') => Some(Action::WalkToWall(Direction::Up)),
            key!('j') => Some(Action::Walk(Direction::Down)),
            key!('J') => Some(Action::WalkToWall(Direction::Down)),
            _ => None,
        };
        if let Some(action) = action {
            sender.send(action);
        }
    }
}
