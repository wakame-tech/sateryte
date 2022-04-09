use std::collections::HashMap;

use bevy::{asset::HandleId, prelude::*};
use bevy_crossterm::components::Sprite;
use terminal_size::{terminal_size, Width};

#[derive(Default, Component)]
pub struct StatusBar {
    pub props: HashMap<String, String>,
    pub handle: Option<HandleId>,
}

impl StatusBar {
    pub fn to_sprite(&self) -> Sprite {
        let (Width(width), _) = terminal_size().unwrap();
        let content = self
            .props
            .iter()
            .map(|(key, value)| format!("{}: {}", key, value))
            .collect::<Vec<String>>()
            .join(" ");
        let content = format!("{:width$}", content, width = width as usize);
        Sprite::new(content)
    }
}

#[derive(Component, Clone)]
pub struct StatusBarUpdateEvent {
    pub key: String,
    pub value: String,
}

impl StatusBarUpdateEvent {
    pub fn new(key: &str, value: &str) -> Self {
        Self {
            key: key.to_string(),
            value: value.to_string(),
        }
    }
}
