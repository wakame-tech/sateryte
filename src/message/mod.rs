use bevy::{app::PluginGroupBuilder, prelude::*};

use self::{logger_plugin::LoggerPlugin, status_bar_plugin::StatusBarPlugin};

pub mod components;
pub mod logger_plugin;
pub mod status_bar_plugin;
pub mod systems;

/// メッセージに関するプラグイン群
pub struct MessagePlugins;

impl PluginGroup for MessagePlugins {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group.add(LoggerPlugin).add(StatusBarPlugin);
    }
}
