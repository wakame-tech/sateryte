use bevy::prelude::*;

use crate::{
    dungeon_world::dungeon::Dungeon, enemy::components::EnemyMovedEvent,
    message::components::status_bar::StatusBarUpdateEvent,
};

use super::components::turn::Turn;

pub fn setup_turn(mut commands: Commands, query: Query<&Dungeon, Added<Dungeon>>) {
    for _ in query.iter() {
        commands.spawn().insert(Turn::new());
    }
}

pub fn increment_turn(mut turn_query: Query<&mut Turn>, mut reader: EventReader<EnemyMovedEvent>) {
    for _ in reader.iter() {
        let mut turn = turn_query.single_mut();
        turn.increment();
    }
}

pub fn render_turn_status(
    turn_query: Query<&Turn, Changed<Turn>>,
    mut status_bar: EventWriter<StatusBarUpdateEvent>,
) {
    for turn in turn_query.iter() {
        status_bar.send(StatusBarUpdateEvent::new(
            "turn",
            turn.0.to_string().as_str(),
        ));
    }
}
