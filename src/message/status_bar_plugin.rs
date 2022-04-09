use bevy::prelude::*;

use super::{
    components::status_bar::{StatusBar, StatusBarUpdateEvent},
    systems::status_bar_listener::status_bar_listener,
};

pub struct StatusBarPlugin;

impl Plugin for StatusBarPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<StatusBarUpdateEvent>()
            .init_resource::<StatusBar>()
            .add_system(status_bar_listener);
    }
}
