use crate::{geo::direction::Direction, player::components::action::Action};
use bevy::{app::Events, prelude::*};
use bevy_crossterm::prelude::{KeyCode, KeyEvent};
use crossterm::event::KeyModifiers;

fn parse_key_binding(binding: &str) -> KeyEvent {
    let with_shift = binding.starts_with("S-");
    let with_ctrl = binding.starts_with("C-");
    let rest = binding.trim_start_matches("S-").trim_start_matches("C-");
    let code = match rest {
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
        // y k u
        // h _ l
        // b j n
        //
        // dash: S-
        let key_bindings: Vec<(Action, Vec<&str>)> = vec![
            // 右
            (Action::Walk(Direction::Right), vec!["l", "<right>"]),
            // 右ダッシュ
            (Action::Dash(Direction::Right), vec!["S-l", "S-<right>"]),
            // 左
            (Action::Walk(Direction::Left), vec!["h", "<left>"]),
            // 左ダッシュ
            (Action::Dash(Direction::Left), vec!["S-h", "S-<left>"]),
            // 上
            (Action::Walk(Direction::Up), vec!["k", "<up>"]),
            // 上ダッシュ
            (Action::Dash(Direction::Up), vec!["S-k", "S-<up>"]),
            // 下
            (Action::Walk(Direction::Down), vec!["j", "<down>"]),
            // 下ダッシュ
            (Action::Dash(Direction::Down), vec!["S-j", "S-<down>"]),
            // 右上
            (Action::Walk(Direction::UpRight), vec!["u"]),
            // 右上ダッシュ
            (Action::Dash(Direction::UpRight), vec!["S-u"]),
            // 左上
            (Action::Walk(Direction::UpLeft), vec!["y"]),
            // 左上ダッシュ
            (Action::Dash(Direction::UpLeft), vec!["S-y"]),
            // 右下
            (Action::Walk(Direction::DownRight), vec!["n"]),
            // 右下ダッシュ
            (Action::Dash(Direction::DownRight), vec!["S-n"]),
            // 左下
            (Action::Walk(Direction::DownLeft), vec!["b"]),
            // 左下ダッシュ
            (Action::Dash(Direction::DownLeft), vec!["S-b"]),
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
