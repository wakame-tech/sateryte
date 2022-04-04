use bevy::prelude::*;
use bevy_crossterm::components::Position;

use crate::{
    geo::point::Point,
    player::components::{action::Action, entity_bundle::IsPlayer, event::PlayerMoveEvent},
    world::dungeon_world::dungeon::Dungeon,
};

/// 入力を受け, プレイヤーの行動をする
pub fn act_player(
    mut reader: EventReader<Action>,
    dungeon_query: Query<&Dungeon>,
    mut player_query: Query<&mut Position, With<IsPlayer>>,
    mut player_move_event: EventWriter<PlayerMoveEvent>,
) {
    for action in reader.iter() {
        if let Some(mut player) = player_query.iter_mut().next() {
            if let Some(dungeon) = dungeon_query.iter().next() {
                let pos = Point::new(player.x, player.y);
                // let enemy_poses = enemy_query
                //     .q1()
                //     .iter()
                //     .map(|p| Point::new(p.x, p.y))
                //     .collect::<Vec<_>>();
                let new_pos = match action {
                    Action::Walk(dir) => {
                        let diff: Point = dir.clone().into();
                        pos + diff
                    }
                    Action::Dash(dir) => {
                        let pos = Point::new(player.x, player.y);
                        dungeon.get_next_wall_pos(pos, dir)
                    }
                };
                // if enemy_poses.contains(&new_pos) {
                //     return;
                // }
                if pos != new_pos && dungeon.is_movable(new_pos) {
                    player.x = new_pos.x;
                    player.y = new_pos.y;
                    player_move_event.send(PlayerMoveEvent);
                }
            }
        }
    }
}
