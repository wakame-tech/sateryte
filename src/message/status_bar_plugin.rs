use bevy::prelude::*;

use crate::{config::SateryteOptions, geo::rect::Rect};

use super::{
    components::status_bar::{StatusBar, StatusBarOptions, StatusBarUpdateEvent},
    systems::status_bar_listener::status_bar_listener,
};

pub struct StatusBarPlugin;

impl Plugin for StatusBarPlugin {
    fn build(&self, app: &mut App) {
        let screen_size = app.world.get_resource::<SateryteOptions>().unwrap().size;
        let area = Rect::new(0, screen_size.h as i32 - 1, screen_size.w, 1);

        app.add_event::<StatusBarUpdateEvent>()
            .insert_resource(StatusBarOptions::new(area))
            .init_resource::<StatusBar>()
            .add_system(status_bar_listener);
    }
}
