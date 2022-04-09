use bevy::prelude::*;
use bevy_crossterm::components::Position;

use crate::{
    geo::direction::Direction,
    message::components::status_bar::StatusBarUpdateEvent,
    player::components::{
        entity_bundle::IsPlayer,
        status::{Exp, Hp},
    },
};

/// プレイヤーのposをステータスバーに描画する
pub fn render_player_position_status(
    mut status_bar: EventWriter<StatusBarUpdateEvent>,
    query: Query<&Position, (With<IsPlayer>, Changed<Position>)>,
) {
    for player in query.iter() {
        status_bar.send(StatusBarUpdateEvent {
            key: "at".to_string(),
            value: format!("({}, {})", player.x, player.y),
        });
    }
}

/// プレイヤーのhpをステータスバーに描画する
pub fn render_player_hp_status(
    mut status_bar: EventWriter<StatusBarUpdateEvent>,
    query: Query<&Hp, (With<IsPlayer>, Changed<Hp>)>,
) {
    for hp in query.iter() {
        status_bar.send(StatusBarUpdateEvent {
            key: "hp".to_string(),
            value: format!("{}/{}", hp.value, hp.max),
        });
    }
}

/// プレイヤーのexpをステータスバーに描画する
pub fn render_player_exp_status(
    mut status_bar: EventWriter<StatusBarUpdateEvent>,
    query: Query<&Exp, (With<IsPlayer>, Changed<Exp>)>,
) {
    for exp in query.iter() {
        status_bar.send(StatusBarUpdateEvent {
            key: "exp".to_string(),
            value: format!("{}/{}", exp.value, exp.next),
        });
    }
}

pub fn render_player_dir_status(
    mut status_bar: EventWriter<StatusBarUpdateEvent>,
    query: Query<&Direction, (With<IsPlayer>, Changed<Direction>)>,
) {
    for dir in query.iter() {
        status_bar.send(StatusBarUpdateEvent {
            key: "dir".to_string(),
            value: format!("{:?}", dir),
        });
    }
}
