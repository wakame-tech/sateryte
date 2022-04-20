use bevy::prelude::*;
use bevy_crossterm::components::Position;

use crate::{
    dungeon_world::dungeon::Dungeon,
    enemy::components::EnemyMovedEvent,
    geo::{direction::Direction, point::Point},
    player::components::{
        action::Action,
        entity_bundle::{Flags, IsPlayer},
        event::PlayerActedEvent,
    },
};

/// 入力を受け, プレイヤーの行動をする
pub fn act_player(
    mut reader: EventReader<Action>,
    dungeon: Option<Res<Dungeon>>,
    mut player_query: Query<(&mut Point, &mut Direction, &mut Flags), With<IsPlayer>>,
    mut on_player_acted: EventWriter<PlayerActedEvent>,
) {
    for action in reader.iter() {
        if let Some(ref dungeon) = dungeon {
            let (mut point, mut dir, mut flags) = player_query.iter_mut().next().unwrap();

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
                Action::Turn(_) => {}
                Action::Walk(d) => {
                    if let Some(new_pos) = dungeon.get_next_pos(*point, d) {
                        *dir = d.clone();
                        *point = new_pos;
                    }
                }
                Action::Dash(d) => {
                    if let Some(new_pos) = dungeon.get_next_pos(*point, d) {
                        if dungeon.is_movable(&new_pos) {
                            flags.is_dash = true;
                        }
                        *dir = d.clone();
                        *point = new_pos;
                    }
                }
            }

            on_player_acted.send(PlayerActedEvent {
                action: action.clone(),
                pos: *point,
            });
            // if enemy_poses.contains(&new_pos) {
            //     return;
            // }
        }
    }
}

/// ダッシュ中の時は自動で次の行動を決定する
pub fn auto_dash(
    dungeon: Option<Res<Dungeon>>,
    mut player_query: Query<(&mut Point, &mut Direction, &mut Flags), With<IsPlayer>>,
    mut player_moved: EventWriter<PlayerActedEvent>,
    mut enemy_moved: EventReader<EnemyMovedEvent>,
) {
    for _ in enemy_moved.iter() {
        if let Some(ref dungeon) = dungeon {
            let (mut point, dir, mut flags) = player_query.single_mut();
            if !flags.is_dash {
                return;
            }

            log::debug!("auto dash @{}", *point);

            // ダッシュをやめるか
            let cancel_dash = dungeon.cancel_dash(&point, dir.as_ref());
            log::debug!("cancel_dash: {}", cancel_dash);

            if cancel_dash {
                flags.is_dash = !cancel_dash;
            } else {
                if let Some(new_pos) = dungeon.get_next_pos(*point, &dir) {
                    *point = new_pos;
                }
            }
            player_moved.send(PlayerActedEvent {
                action: Action::Dash(dir.clone()),
                pos: point.clone(),
            });
        }
    }
}

/// [Player] 座標を画面上の座標に反映させる
pub fn render_player(
    mut player_query: Query<(&Flags, &mut Position, &Point), (With<IsPlayer>, Changed<Point>)>,
) {
    for (flags, mut position, point) in player_query.iter_mut() {
        log::debug!("render_player: {}, is_dash = {}", *point, flags.is_dash);
        // ダッシュ中は描画させない
        if !flags.is_dash {
            position.x = point.x;
            position.y = point.y;
        }
    }
}
