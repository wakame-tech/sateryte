use core::time;

use bevy::{app::ScheduleRunnerSettings, prelude::*};
use bevy_crossterm::{CrosstermWindowSettings, DefaultCrosstermPlugins};
use sateryte::{input::input_keys::input_keys, message::MessagePlugin, world::WorldPlugin};

fn main() -> Result<(), anyhow::Error> {
    let mut settings = CrosstermWindowSettings::default();
    settings.set_title("satellite-rs");

    App::new()
        .insert_resource(settings)
        .insert_resource(DefaultTaskPoolOptions::with_num_threads(1))
        .insert_resource(ScheduleRunnerSettings::run_loop(
            time::Duration::from_millis(16),
        ))
        .add_plugins(DefaultCrosstermPlugins)
        .add_plugin(MessagePlugin)
        .add_plugin(WorldPlugin)
        .add_system(input_keys)
        .run();

    Ok(())
}
