use bevy::prelude::*;
use bevy_crossterm::components::Position;

use crate::{
    geo::point::Point, message::status_bar::StatusBarUpdateEvent, world::dungeon::Dungeon,
};

use super::{actions::Action, Player};

pub fn world_action_listener(
    mut reader: EventReader<Action>,
    dungeon_query: Query<&Dungeon>,
    mut player_query: Query<(&Player, &mut Position)>,
    mut status_bar: EventWriter<StatusBarUpdateEvent>,
) {
    for action in reader.iter() {
        if let Some((_, mut player)) = player_query.iter_mut().next() {
            if let Some(dungeon) = dungeon_query.iter().next() {
                let pos = Point::new(player.x, player.y);
                let new_pos = match action {
                    Action::Walk(dir) => {
                        let diff: Point = dir.clone().into();
                        pos + diff
                    }
                    Action::WalkToWall(dir) => {
                        let pos = Point::new(player.x, player.y);
                        dungeon.get_next_wall_pos(pos, dir)
                    }
                };
                if pos != new_pos && dungeon.is_movable(new_pos) {
                    player.x = new_pos.x;
                    player.y = new_pos.y;
                }

                status_bar.send(StatusBarUpdateEvent {
                    key: "@".to_string(),
                    value: format!("({}, {})", player.x, player.y),
                });
            }
        }
    }
}
