use bevy::prelude::*;

use crate::{
    dungeon_world::dungeon::Dungeon, enemy::components::EnemyMovedEvent,
    player::components::event::PlayerActedEvent,
};

pub fn enemy_move(
    mut player_moved: EventReader<PlayerActedEvent>,
    mut enemy_moved: EventWriter<EnemyMovedEvent>,
    dungeon: Option<Res<Dungeon>>,
) {
    for _ in player_moved.iter() {
        if let Some(_) = dungeon {
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
}
