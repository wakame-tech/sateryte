use std::collections::HashMap;

use crate::geo::rect::Rect;
use bevy::{asset::HandleId, prelude::*};
use bevy_crossterm::components::Sprite;
use log;
use terminal_size::{terminal_size, Width};

#[derive(Default, Component)]
pub struct StatusBar {
    pub props: HashMap<String, String>,
    pub handle: Option<HandleId>,
}

#[derive(Component)]
pub struct StatusBarOptions {
    /// 表示領域
    pub area: Rect,
}

impl StatusBarOptions {
    pub fn new(area: Rect) -> Self {
        log::debug!("status bar area: {}", area);
        Self { area }
    }
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
