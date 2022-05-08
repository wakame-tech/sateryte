use self::keybinding::keyboard::KeyboardKeyBindings;
use crate::geo::size::Size;
use serde::{Deserialize, Serialize};
use std::fs::read_to_string;
use terminal_size::{terminal_size, Height, Width};
pub mod keybinding;

#[derive(Debug, Serialize, Deserialize)]
pub struct SateryteConfig {
    /// フロアマップ領域
    pub floor: Option<(i32, i32, usize, usize)>,
    /// ステータスバー領域
    pub status_bar: Option<(i32, i32, usize, usize)>,
    /// メッセージ領域
    pub message: Option<(i32, i32, usize, usize)>,
    /// デバッグモードか
    pub is_debug: bool,
    /// キーボード入力設定
    pub key_binding: KeyboardKeyBindings,
}

pub fn get_terminal_size() -> Size {
    let (Width(w), Height(h)) = terminal_size().unwrap();
    Size::new(w as usize, h as usize)
}

impl SateryteConfig {
    pub fn from_file(path: &str) -> Result<Self, anyhow::Error> {
        let content = read_to_string(path)?;
        let config: SateryteConfig = ron::from_str(&content)?;
        let default_config = SateryteConfig::default();
        Ok(SateryteConfig {
            floor: config.floor.or(default_config.floor),
            status_bar: config.status_bar.or(default_config.status_bar),
            message: config.message.or(default_config.message),
            is_debug: config.is_debug,
            key_binding: config.key_binding,
        })
    }
}

impl Default for SateryteConfig {
    fn default() -> Self {
        // set screen size
        let screen_size = get_terminal_size();
        SateryteConfig {
            floor: Some((screen_size - Size::new(0, 1)).at(0, 0).as_tuple()),
            status_bar: Some((0, 0, screen_size.w, 1)),
            message: Some((80, 0, 20, 60)),
            is_debug: false,
            key_binding: KeyboardKeyBindings::default(),
        }
    }
}
