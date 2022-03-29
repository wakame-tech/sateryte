use bevy::prelude::*;

use self::{
    logger::{draw_logger, logger_listener, setup_logger, LogEvent},
    status_bar::{draw_status_bar, setup_status_bar, status_bar_listener, StatusBarUpdateEvent},
};

pub mod logger;
pub mod status_bar;

pub struct MessagePlugin;

impl Plugin for MessagePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<LogEvent>()
            .add_event::<StatusBarUpdateEvent>()
            .add_startup_system(setup_logger)
            .add_startup_system(setup_status_bar)
            .add_system(logger_listener)
            .add_system(status_bar_listener)
            .add_system(draw_logger)
            .add_system(draw_status_bar);
    }
}
