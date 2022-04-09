use bevy::prelude::*;

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
