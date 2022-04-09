use bevy::prelude::*;

use super::{
    components::logger::{LogEvent, Logger, LoggerOptions},
    systems::logger_listener::logger_listener,
};

/// メッセージウインドウ
pub struct LoggerPlugin;

impl Plugin for LoggerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<LogEvent>()
            .init_resource::<LoggerOptions>()
            .init_resource::<Logger>()
            .add_system(logger_listener);
    }
}
