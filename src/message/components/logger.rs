use crate::geo::rect::Rect;
use bevy::prelude::*;
use log;

#[derive(Component, Default)]
pub struct Logger {
    pub messages: Vec<LogEvent>,
}

#[derive(Component)]
pub struct LoggerOptions {
    /// 表示領域
    pub area: Rect,
}

impl Default for LoggerOptions {
    fn default() -> Self {
        let area = Rect::new(80, 0, 20, 60);
        log::debug!("logger {}", area);
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
