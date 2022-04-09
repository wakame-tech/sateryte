use crate::{geo::direction::Direction, player::components::action::Action};
use bevy::{app::Events, prelude::*};
use bevy_crossterm::prelude::{KeyCode, KeyEvent};
use crossterm::event::KeyModifiers;

fn parse_key_binding(binding: &str) -> KeyEvent {
    let with_shift = binding.starts_with("S-");
    let with_ctrl = binding.starts_with("C-");
    let rest = binding.trim_start_matches("S-").trim_start_matches("C-");
    let code = match rest {
        "<space>" => KeyCode::Char(' '),
        "<up>" => KeyCode::Up,
        "<down>" => KeyCode::Down,
        "<left>" => KeyCode::Left,
        "<right>" => KeyCode::Right,
        _ => KeyCode::Char(rest.chars().next().unwrap()),
    };
    let modifiers = if with_shift {
        KeyModifiers::SHIFT
    } else if with_ctrl {
        KeyModifiers::CONTROL
    } else {
        KeyModifiers::NONE
    };
    KeyEvent::new(code, modifiers)
}

/// convert input key to action
pub fn input_keys(keys: Res<Events<KeyEvent>>, mut sender: EventWriter<Action>) {
    for event in keys.get_reader().iter(&*keys) {
        // q w e
        // a _ d
        // z s v
        //
        // dash: S-
        // turn: C-
        let key_bindings: Vec<(Action, Vec<&str>)> = vec![
            // 足踏み
            (Action::Step, vec!["<space>"]),
            // 右方向転換
            (Action::Turn(Direction::Right), vec!["C-d"]),
            // 左方向転換
            (Action::Turn(Direction::Left), vec!["C-a"]),
            // 上方向転換
            (Action::Turn(Direction::Up), vec!["C-w"]),
            // 下方向転換
            (Action::Turn(Direction::Down), vec!["C-s"]),
            // 右上方向転換
            (Action::Turn(Direction::UpRight), vec!["C-e"]),
            // 右下方向転換
            (Action::Turn(Direction::DownRight), vec!["C-v"]),
            // 左上方向転換
            (Action::Turn(Direction::UpLeft), vec!["C-q"]),
            // 左下方向転換
            (Action::Turn(Direction::DownLeft), vec!["C-z"]),
            // 右
            (Action::Walk(Direction::Right), vec!["d"]),
            // 左
            (Action::Walk(Direction::Left), vec!["a"]),
            // 上
            (Action::Walk(Direction::Up), vec!["w"]),
            // 下
            (Action::Walk(Direction::Down), vec!["s"]),
            // 右上
            (Action::Walk(Direction::UpRight), vec!["e"]),
            // 左上
            (Action::Walk(Direction::UpLeft), vec!["q"]),
            // 右下
            (Action::Walk(Direction::DownRight), vec!["v"]),
            // 左下
            (Action::Walk(Direction::DownLeft), vec!["z"]),
            // 右ダッシュ
            (Action::Dash(Direction::Right), vec!["S-d"]),
            // 左ダッシュ
            (Action::Dash(Direction::Left), vec!["S-a"]),
            // 上ダッシュ
            (Action::Dash(Direction::Up), vec!["S-w"]),
            // 下ダッシュ
            (Action::Dash(Direction::Down), vec!["S-s"]),
            // 右上ダッシュ
            (Action::Dash(Direction::UpRight), vec!["S-e"]),
            // 左上ダッシュ
            (Action::Dash(Direction::UpLeft), vec!["S-q"]),
            // 右下ダッシュ
            (Action::Dash(Direction::DownRight), vec!["S-v"]),
            // 左下ダッシュ
            (Action::Dash(Direction::DownLeft), vec!["S-z"]),
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
