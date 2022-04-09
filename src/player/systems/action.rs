use bevy::prelude::*;
use bevy_crossterm::components::Position;

use crate::{
    dungeon_world::dungeon::Dungeon,
    enemy::components::EnemyMovedEvent,
    geo::{direction::Direction, point::Point},
    message::components::logger::LogEvent,
    player::components::{
        action::Action,
        entity_bundle::{Flags, IsPlayer},
        event::PlayerMovedEvent,
    },
};

/// 入力を受け, プレイヤーの行動をする
/// FIXME: 幅2マスでダッシュするとダッシュ終わりに描画が更新されない
pub fn act_player(
    mut reader: EventReader<Action>,
    dungeon_query: Query<&Dungeon>,
    mut player_query: Query<(&mut Point, &mut Direction, &mut Flags), With<IsPlayer>>,
    mut player_move_event: EventWriter<PlayerMovedEvent>,
) {
    for action in reader.iter() {
        let dungeon = dungeon_query.iter().next();
        if dungeon.is_none() {
            continue;
        }
        let dungeon = dungeon.unwrap();
        let (mut point, mut dir, mut flags) = player_query.single_mut();

        if flags.is_dash {
            continue;
        }

        // let enemy_poses = enemy_query
        //     .q1()
        //     .iter()
        //     .map(|p| Point::new(p.x, p.y))
        //     .collect::<Vec<_>>();

        match action {
            Action::Step => {}
            Action::Walk(d) => {
                if let Some(new_pos) = dungeon.get_next_pos(*point, d) {
                    *dir = d.clone();
                    *point = new_pos;
                    player_move_event.send(PlayerMovedEvent);
                }
            }
            Action::Dash(d) => {
                if let Some(new_pos) = dungeon.get_next_pos(*point, d) {
                    flags.is_dash = true;
                    *dir = d.clone();
                    *point = new_pos;
                    player_move_event.send(PlayerMovedEvent);
                }
            }
        }
        // if enemy_poses.contains(&new_pos) {
        //     return;
        // }
    }
}

/// ダッシュ中の時は自動で次の行動を決定する
pub fn auto_dash(
    dungeon_query: Query<&Dungeon>,
    mut player_query: Query<(&mut Point, &mut Direction, &mut Flags), With<IsPlayer>>,
    mut player_moved: EventWriter<PlayerMovedEvent>,
    mut enemy_moved: EventReader<EnemyMovedEvent>,
    mut logger: EventWriter<LogEvent>,
) {
    for _ in enemy_moved.iter() {
        // logger.send(LogEvent::info("auto_dash"));
        let (mut point, mut dir, mut flags) = player_query.single_mut();
        if !flags.is_dash {
            return;
        }
        let dungeon = dungeon_query.single();

        if let Some(new_pos) = dungeon.get_next_pos(*point, &dir) {
            point.x = new_pos.x;
            point.y = new_pos.y;
        }
        flags.is_dash = !dungeon.cancel_dash(&point, dir.as_ref());
        player_moved.send(PlayerMovedEvent);
        // logger.send(LogEvent::info(format!("{:?}", *point).as_str()))
    }
}

/// デバッグ用
pub fn debug_player_action(
    mut player_moved: EventReader<PlayerMovedEvent>,
    value: Query<&Flags, With<IsPlayer>>,
    mut logger: EventWriter<LogEvent>,
) {
    for _ in player_moved.iter() {
        let v = value.single();
        logger.send(LogEvent::info(format!("{:?}", v).as_str()))
    }
}

/// [Player] 座標を画面上の座標に反映させる
pub fn render_player(
    mut player_query: Query<
        (&mut Position, &mut Point, &mut Direction, &mut Flags),
        (With<IsPlayer>, Changed<Point>),
    >,
) {
    for (mut position, point, _, flags) in player_query.iter_mut() {
        // ダッシュ中は負荷軽減のため, 座標反映させない
        if !flags.is_dash {
            position.x = point.x;
            position.y = point.y;
        }
    }
}
