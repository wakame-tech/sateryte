use bevy::prelude::*;

use crate::{
    dungeon_world::dungeon::Dungeon,
    enemy::components::{Enemy, EnemyMovedEvent},
    message::logger::LogEvent,
    player::components::event::PlayerMovedEvent,
};
use bevy_crossterm::components::Position;

pub fn enemy_move(
    mut player_moved: EventReader<PlayerMovedEvent>,
    mut enemy_moved: EventWriter<EnemyMovedEvent>,
    dungeon_query: Query<&Dungeon>,
    mut enemy_query: Query<(&Enemy, &mut Position)>,
    mut logger: EventWriter<LogEvent>,
) {
    for _ in player_moved.iter() {
        let dungeon = dungeon_query.single();

        // for (enemy, mut position) in enemy_query.iter_mut() {
        //     let mut rng = rand::thread_rng();
        //     let new_pos = dungeon.get_next_pos(
        //         Point::new(position.x, position.y),
        //         Direction::around_4().choose(&mut rng).unwrap(),
        //     );
        //     if new_pos != Point::new(position.x, position.y) {
        //         position.x = new_pos.x;
        //         position.y = new_pos.y;
        //     }
        // }

        enemy_moved.send(EnemyMovedEvent);
        // logger.send(LogEvent::info("enemy moved"));
    }
}
