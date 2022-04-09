use bevy::{app::ScheduleRunnerSettings, log::LogPlugin, prelude::*};
use bevy_crossterm::{CrosstermWindowSettings, DefaultCrosstermPlugins};
use core::time;
use log;
use sateryte::{
    config::SateryteOptions,
    geo::size::Size,
    input::input_plugin::KeyBoardInputPlugin,
    message::MessagePlugins,
    world::{components::event::WorldGenerateEvent, world_plugin::WorldPlugin},
};

/// エントリポイント
fn start(mut writer: EventWriter<WorldGenerateEvent>) {
    let event = WorldGenerateEvent {
        world_size: Size::new(80, 25),
        world_name: "test".to_string(),
        floor: 1,
    };
    writer.send(event);
}

fn init_logger() -> Result<(), anyhow::Error> {
    let file_config = fern::Dispatch::new()
        .level(log::LevelFilter::Debug)
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}] {}",
                chrono::Local::now().format("[%Y-%m-%d %H:%M:%S]"),
                record.level(),
                message
            ));
        })
        .chain(fern::log_file("debug.log").unwrap());

    file_config.apply().map_err(|e| anyhow::anyhow!("{}", e))
}

fn main() -> Result<(), anyhow::Error> {
    let mut settings = CrosstermWindowSettings::default();
    init_logger()?;
    log::debug!("logger initialized");

    settings.set_title("satellite-rs");

    App::new()
        .insert_resource(SateryteOptions::default())
        .insert_resource(settings)
        .insert_resource(DefaultTaskPoolOptions::with_num_threads(1))
        .insert_resource(ScheduleRunnerSettings::run_loop(
            time::Duration::from_millis(16),
        ))
        .add_plugins_with(DefaultCrosstermPlugins, |group| {
            group.disable::<LogPlugin>()
        })
        .add_plugin(KeyBoardInputPlugin)
        .add_plugins(MessagePlugins)
        .add_plugin(WorldPlugin)
        .add_startup_system(start)
        .run();

    Ok(())
}
