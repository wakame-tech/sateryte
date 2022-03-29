use bevy::prelude::*;
use bevy_crossterm::components::Position;
use crossterm::style;

use crate::{geo::point::Point, message::Message, world::dungeon::Dungeon};

use super::{actions::Action, Player};

pub fn world_action_listener(
    mut reader: EventReader<Action>,
    dungeon_query: Query<&Dungeon>,
    mut player_query: Query<(&Player, &mut Position)>,
    mut message_writer: EventWriter<Message>,
) {
    for action in reader.iter() {
        if let Some((_, mut player)) = player_query.iter_mut().next() {
            if let Some(dungeon) = dungeon_query.iter().next() {
                // dbg!(&dungeon.areas);
                match action {
                    Action::Walk(dir) => {
                        let pos = Point::new(player.x, player.y);
                        let diff: Point = dir.clone().into();
                        let new_pos = pos + diff;
                        if dungeon.is_movable(new_pos) {
                            player.x = new_pos.x;
                            player.y = new_pos.y;
                            let text = format!("[move] ({}, {})", new_pos.x, new_pos.y);
                            message_writer.send(Message { text });
                        }
                    }
                    Action::WalkToWall(dir) => {
                        message_writer.send(Message {
                            text: format!("{:?}", dir),
                        });
                        let pos = Point::new(player.x, player.y);
                        let new_pos = dungeon.get_next_wall_pos(pos, dir);
                        if dungeon.is_movable(new_pos) {
                            player.x = new_pos.x;
                            player.y = new_pos.y;
                            let text = format!("[move] ({}, {})", new_pos.x, new_pos.y);
                            message_writer.send(Message { text });
                        }
                    }
                }
            }
        }
    }
}
