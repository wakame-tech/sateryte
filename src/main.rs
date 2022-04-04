use core::time;

use bevy::{app::ScheduleRunnerSettings, prelude::*};
use bevy_crossterm::{CrosstermWindowSettings, DefaultCrosstermPlugins};
use sateryte::{
    geo::size::Size,
    input::input_plugin::KeyBoardInputPlugin,
    message::MessagePlugins,
    player::player_plugin::PlayerPlugins,
    world::{components::event::WorldGenerateEvent, world_plugin::WorldPlugin},
};

fn start(mut writer: EventWriter<WorldGenerateEvent>) {
    let event = WorldGenerateEvent {
        world_size: Size::new(80, 25),
        world_name: "test".to_string(),
        floor: 1,
    };
    writer.send(event);
}

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
        .add_plugin(KeyBoardInputPlugin)
        .add_plugins(MessagePlugins)
        .add_plugin(WorldPlugin)
        .add_startup_system(start)
        .run();

    Ok(())
}
