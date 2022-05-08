use crate::{
    config::{keybinding::parser::parse_key_binding, SateryteConfig},
    geo::direction::Direction,
    player::components::action::Action,
};
use bevy::ecs::event::Events;
use bevy::prelude::*;
use bevy_crossterm::prelude::KeyEvent;

/// convert input key to action
pub fn input_keys(
    keys: Res<Events<KeyEvent>>,
    mut sender: EventWriter<Action>,
    config: Res<SateryteConfig>,
) {
    for event in keys.get_reader().iter(&*keys) {
        let key_bindings: Vec<(Action, &[String])> = vec![
            // 足踏み
            (Action::Step, &*config.key_binding.step),
            // 右方向転換
            (
                Action::Turn(Direction::Right),
                &*config.key_binding.turn_right,
            ),
            // 左方向転換
            (
                Action::Turn(Direction::Left),
                &*config.key_binding.turn_left,
            ),
            // 上方向転換
            (Action::Turn(Direction::Up), &*config.key_binding.turn_up),
            // 下方向転換
            (
                Action::Turn(Direction::Down),
                &*config.key_binding.turn_down,
            ),
            // 右上方向転換
            (
                Action::Turn(Direction::UpRight),
                &*config.key_binding.turn_up_right,
            ),
            // 右下方向転換
            (
                Action::Turn(Direction::DownRight),
                &*config.key_binding.turn_down_right,
            ),
            // 左上方向転換
            (
                Action::Turn(Direction::UpLeft),
                &*config.key_binding.turn_up_left,
            ),
            // 左下方向転換
            (
                Action::Turn(Direction::DownLeft),
                &*config.key_binding.turn_down_left,
            ),
            // 右
            (Action::Walk(Direction::Right), &*config.key_binding.right),
            // 左
            (Action::Walk(Direction::Left), &*config.key_binding.left),
            // 上
            (Action::Walk(Direction::Up), &*config.key_binding.up),
            // 下
            (Action::Walk(Direction::Down), &*config.key_binding.down),
            // 右上
            (
                Action::Walk(Direction::UpRight),
                &*config.key_binding.up_right,
            ),
            // 左上
            (
                Action::Walk(Direction::UpLeft),
                &*config.key_binding.up_left,
            ),
            // 右下
            (
                Action::Walk(Direction::DownRight),
                &*config.key_binding.down_right,
            ),
            // 左下
            (
                Action::Walk(Direction::DownLeft),
                &*config.key_binding.down_left,
            ),
            // 右ダッシュ
            (
                Action::Dash(Direction::Right),
                &*config.key_binding.dash_right,
            ),
            // 左ダッシュ
            (
                Action::Dash(Direction::Left),
                &*config.key_binding.dash_left,
            ),
            // 上ダッシュ
            (Action::Dash(Direction::Up), &*config.key_binding.dash_up),
            // 下ダッシュ
            (
                Action::Dash(Direction::Down),
                &*config.key_binding.dash_down,
            ),
            // 右上ダッシュ
            (
                Action::Dash(Direction::UpRight),
                &*config.key_binding.dash_up_right,
            ),
            // 左上ダッシュ
            (
                Action::Dash(Direction::UpLeft),
                &*config.key_binding.dash_up_left,
            ),
            // 右下ダッシュ
            (
                Action::Dash(Direction::DownRight),
                &*config.key_binding.dash_down_right,
            ),
            // 左下ダッシュ
            (
                Action::Dash(Direction::DownLeft),
                &*config.key_binding.dash_down_left,
            ),
        ];

        for (action, bindings) in key_bindings.iter() {
            if bindings
                .iter()
                .map(|bind| parse_key_binding(bind))
                .any(|key| key == *event)
            {
                sender.send(action.to_owned());
            }
        }
    }
}
