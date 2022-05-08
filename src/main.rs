use bevy::{app::ScheduleRunnerSettings, log::LogPlugin, prelude::*};
use bevy_crossterm::{CrosstermWindowSettings, DefaultCrosstermPlugins};
use core::time;
use log;
use sateryte::{
    config::SateryteConfig,
    geo::rect::Rect,
    input::input_plugin::KeyBoardInputPlugin,
    message::MessagePlugins,
    world::{components::event::FloorGenerateEvent, world_plugin::WorldPlugin},
};

/// エントリポイント
fn start(mut writer: EventWriter<FloorGenerateEvent>, options: Res<SateryteConfig>) {
    let event = FloorGenerateEvent {
        map_size: Rect::from_tuple(options.floor.unwrap()).size,
        dungeon_name: "test".to_string(),
        floor: 1,
    };
    writer.send(event);
}

fn init_logger(log_file: &str) -> Result<(), anyhow::Error> {
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
        .chain(fern::log_file(log_file).unwrap());

    file_config.apply().map_err(|e| anyhow::anyhow!("{}", e))
}

fn main() -> Result<(), anyhow::Error> {
    // init logger
    init_logger("debug.log")?;
    debug!("logger initialized");

    let config = SateryteConfig::from_file("config.ron")?;
    debug!("config: {:?}", config);

    // set title
    let mut settings = CrosstermWindowSettings::default();
    settings.set_title("satellite-rs");

    App::new()
        .insert_resource(config)
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
