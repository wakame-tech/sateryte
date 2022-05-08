use bevy::prelude::*;

use crate::{config::SateryteConfig, geo::rect::Rect};

use super::{
    components::status_bar::{StatusBar, StatusBarOptions, StatusBarUpdateEvent},
    systems::status_bar_listener::status_bar_listener,
};

pub struct StatusBarPlugin;

impl Plugin for StatusBarPlugin {
    fn build(&self, app: &mut App) {
        let config = app.world.get_resource::<SateryteConfig>().unwrap();
        let status_bar_area = Rect::from_tuple(config.status_bar.unwrap());

        app.add_event::<StatusBarUpdateEvent>()
            .insert_resource(StatusBarOptions::new(status_bar_area))
            .init_resource::<StatusBar>()
            .add_system(status_bar_listener);
    }
}
