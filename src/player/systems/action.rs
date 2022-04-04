use bevy::prelude::*;
use bevy_crossterm::components::Position;

use crate::{
    enemy::EnemyMovedEvent,
    geo::{direction::Direction, point::Point},
    message::logger::LogEvent,
    player::components::{
        action::Action,
        entity_bundle::{Flags, IsPlayer},
        event::PlayerMovedEvent,
    },
    world::dungeon_world::dungeon::Dungeon,
};

/// 入力を受け, プレイヤーの行動をする
pub fn act_player(
    mut reader: EventReader<Action>,
    dungeon_query: Query<&Dungeon>,
    mut player_query: Query<(&mut Position, &mut Direction, &mut Flags), With<IsPlayer>>,
    mut player_move_event: EventWriter<PlayerMovedEvent>,
) {
    for action in reader.iter() {
        let dungeon = dungeon_query.iter().next();
        if dungeon.is_none() {
            continue;
        }
        let dungeon = dungeon.unwrap();
        let (mut pos, mut dir, mut flags) = player_query.single_mut();

        if flags.is_dash {
            continue;
        }

        // let enemy_poses = enemy_query
        //     .q1()
        //     .iter()
        //     .map(|p| Point::new(p.x, p.y))
        //     .collect::<Vec<_>>();

        let point = Point::new(pos.x, pos.y);
        if let Some((new_pos, new_dir)) = match action {
            Action::Step => None,
            Action::Walk(dir) => dungeon.get_next_pos(point, dir),
            Action::Dash(dir) => {
                flags.is_dash = true;
                dungeon.get_next_pos(point, dir)
            }
        } {
            *dir = new_dir.clone();
            pos.x = new_pos.x;
            pos.y = new_pos.y;
        }
        // if enemy_poses.contains(&new_pos) {
        //     return;
        // }
        player_move_event.send(PlayerMovedEvent);
    }
}

/// ダッシュ中の時は自動で次の行動を決定する
pub fn auto_dash(
    dungeon_query: Query<&Dungeon>,
    mut player_query: Query<(&mut Position, &mut Direction, &mut Flags), With<IsPlayer>>,
    mut player_moved: EventWriter<PlayerMovedEvent>,
    mut enemy_moved: EventReader<EnemyMovedEvent>,
) {
    for _ in enemy_moved.iter() {
        let dungeon = dungeon_query.single();
        let (mut pos, mut dir, mut flags) = player_query.single_mut();
        if !flags.is_dash {
            continue;
        }

        let point = Point::new(pos.x, pos.y);
        if let Some((new_pos, new_dir)) = dungeon.get_next_pos(point, &dir) {
            *dir = new_dir.clone();
            pos.x = new_pos.x;
            pos.y = new_pos.y;
        }
        flags.is_dash = !dungeon.cancel_dash(&Point::new(pos.x, pos.y), dir.as_ref());

        player_moved.send(PlayerMovedEvent);
    }
}

/// デバッグ用
pub fn debug_player_action(
    mut player_moved: EventReader<PlayerMovedEvent>,
    player_pos: Query<&Position, With<IsPlayer>>,
    mut logger: EventWriter<LogEvent>,
) {
    for _ in player_moved.iter() {
        let pos = player_pos.single();
        logger.send(LogEvent::info(format!("{:?}", pos).as_str()))
    }
}
