use crate::geo::rect::Rect;
use bevy::prelude::*;

#[derive(Component, Default)]
pub struct Logger {
    pub messages: Vec<LogEvent>,
}

#[derive(Component)]
pub struct LoggerOptions {
    /// 表示領域
    pub area: Rect,
}

impl LoggerOptions {
    pub fn new(area: Rect) -> Self {
        Self { area }
    }
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
