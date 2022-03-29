use core::time;

use bevy::{app::ScheduleRunnerSettings, prelude::*};
use bevy_crossterm::{CrosstermWindowSettings, DefaultCrosstermPlugins};
use sateryte::{
    input::{actions::Action, input_keys::input_keys},
    player::{action_listener, spawn_player},
    world::map::startup_map,
};

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
        .add_event::<Action>()
        .add_startup_system(startup_map)
        .add_startup_system(spawn_player)
        .add_system(input_keys)
        .add_system(action_listener)
        .run();

    Ok(())
}
