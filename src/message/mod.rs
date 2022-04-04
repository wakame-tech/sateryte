use bevy::{app::PluginGroupBuilder, prelude::*};

use self::{
    logger::{logger_listener, LogEvent, Logger},
    status_bar::{status_bar_listener, StatusBar, StatusBarUpdateEvent},
};

pub mod logger;
pub mod status_bar;

/// メッセージウインドウ
pub struct LoggerPlugin;

impl Plugin for LoggerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<LogEvent>()
            .init_resource::<Logger>()
            .add_system(logger_listener);
    }
}

pub struct StatusBarPlugin;

impl Plugin for StatusBarPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<StatusBarUpdateEvent>()
            .init_resource::<StatusBar>()
            .add_system(status_bar_listener);
    }
}

/// メッセージに関するプラグイン群
pub struct MessagePlugins;

impl PluginGroup for MessagePlugins {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group.add(LoggerPlugin).add(StatusBarPlugin);
    }
}
