use bevy::{app::PluginGroupBuilder, prelude::*};

use self::{
    logger::{draw_logger, logger_listener, setup_logger, LogEvent},
    status_bar::{draw_status_bar, setup_status_bar, status_bar_listener, StatusBarUpdateEvent},
};

pub mod logger;
pub mod status_bar;

/// メッセージウインドウ
pub struct LoggerPlugin;

impl Plugin for LoggerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<LogEvent>()
            .add_startup_system(setup_logger)
            .add_system(logger_listener)
            .add_system(draw_logger);
    }
}

pub struct StatusBarPlugin;

impl Plugin for StatusBarPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<StatusBarUpdateEvent>()
            .add_startup_system(setup_status_bar)
            .add_system(status_bar_listener)
            .add_system(draw_status_bar);
    }
}

/// メッセージに関するプラグイン群
pub struct MessagePlugins;

impl PluginGroup for MessagePlugins {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group.add(LoggerPlugin).add(StatusBarPlugin);
    }
}
