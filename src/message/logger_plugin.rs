use bevy::prelude::*;

use crate::config::SateryteConfig;

use super::{
    components::logger::{LogEvent, Logger, LoggerOptions},
    systems::logger_listener::logger_listener,
};
use crate::geo::rect::Rect;

/// メッセージウインドウ
pub struct LoggerPlugin;

impl Plugin for LoggerPlugin {
    fn build(&self, app: &mut App) {
        let config = app.world.get_resource::<SateryteConfig>().unwrap();
        let logger_area = Rect::from_tuple(config.message.unwrap());

        app.add_event::<LogEvent>()
            .insert_resource(LoggerOptions::new(logger_area))
            .init_resource::<Logger>()
            .add_system(logger_listener);
    }
}
