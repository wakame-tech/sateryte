use std::time;

use bevy::{
    app::ScheduleRunnerSettings,
    core::DefaultTaskPoolOptions,
    ecs::event::Events,
    prelude::{App, Assets, Commands, Component, EventReader, EventWriter, Query, Res, ResMut},
};
use bevy_crossterm::{
    components::{Position, Sprite, SpriteBundle, StyleMap},
    prelude::{KeyCode, KeyEvent},
    CrosstermWindowSettings, DefaultCrosstermPlugins,
};

#[derive(Debug, Component)]
pub enum Action {
    Right,
    Left,
    Up,
    Down,
}

fn input_keys(keys: Res<Events<KeyEvent>>, mut sender: EventWriter<Action>) {
    for event in keys.get_reader().iter(&*keys) {
        match event {
            &KeyEvent {
                code: KeyCode::Char('d'),
                modifiers: _,
            } => {
                sender.send(Action::Right);
            }
            &KeyEvent {
                code: KeyCode::Char('a'),
                modifiers: _,
            } => {
                sender.send(Action::Left);
            }
            &KeyEvent {
                code: KeyCode::Char('w'),
                modifiers: _,
            } => {
                sender.send(Action::Up);
            }
            &KeyEvent {
                code: KeyCode::Char('s'),
                modifiers: _,
            } => {
                sender.send(Action::Down);
            }
            _ => {}
        }
    }
}

fn action_listener(mut reader: EventReader<Action>, mut query: Query<(&Player, &mut Position)>) {
    for action in reader.iter() {
        let (_, mut player) = query.single_mut();
        match action {
            Action::Right => {
                player.x += 1;
            }
            Action::Left => {
                player.x -= 1;
            }
            Action::Up => {
                player.y -= 1;
            }
            Action::Down => {
                player.y += 1;
            }
        }
    }
}

#[derive(Component)]
pub struct Player;

fn startup_player(
    mut commands: Commands,
    mut sprites: ResMut<Assets<Sprite>>,
    mut stylemaps: ResMut<Assets<StyleMap>>,
) {
    let player = sprites.add(Sprite::new("@"));
    let color = stylemaps.add(StyleMap::default());

    commands
        .spawn_bundle(SpriteBundle {
            sprite: player,
            position: Position::with_xy(0, 0),
            stylemap: color,
            ..Default::default()
        })
        .insert(Player);
}

pub fn main() {
    let mut settings = CrosstermWindowSettings::default();
    settings.set_title("Window example");

    App::new()
        .insert_resource(settings)
        .insert_resource(DefaultTaskPoolOptions::with_num_threads(1))
        .insert_resource(ScheduleRunnerSettings::run_loop(
            time::Duration::from_millis(50),
        ))
        .add_plugins(DefaultCrosstermPlugins)
        .add_startup_system(startup_player)
        .add_event::<Action>()
        .add_system(input_keys)
        .add_system(action_listener)
        .run();
}
