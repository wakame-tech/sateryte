use bevy_crossterm::prelude::{KeyCode, KeyEvent};
use crossterm::event::KeyModifiers;

///
/// keybinding config parser
///
/// `S-?`: shift + key
/// `C-?`: ctrl + key
/// `<up>, <down>, <left>, <right>`: arrow key
/// `<space>`: space key
pub fn parse_key_binding(binding: &str) -> KeyEvent {
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
