use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct KeyboardKeyBindings {
    /// 足踏み
    /// default: `<space>`
    pub step: Vec<String>,

    /// 右方向転換
    /// default: C-d
    pub turn_right: Vec<String>,

    /// 左方向転換
    /// default: C-a
    pub turn_left: Vec<String>,

    /// 上方向転換
    /// default: C-w
    pub turn_up: Vec<String>,

    /// 下方向転換
    /// default: C-s
    pub turn_down: Vec<String>,

    /// 右上方向転換
    /// default: C-e
    pub turn_up_right: Vec<String>,

    /// 右下方向転換
    /// default: C-v
    pub turn_down_right: Vec<String>,

    /// 左上方向転換
    /// default: C-q
    pub turn_up_left: Vec<String>,

    /// 左下方向転換
    /// default: C-z
    pub turn_down_left: Vec<String>,

    /// 右
    /// default: `d`
    pub right: Vec<String>,

    /// 左
    /// default: `a`
    pub left: Vec<String>,

    /// 上
    /// default: `w`
    pub up: Vec<String>,

    /// 下
    /// default: `s`
    pub down: Vec<String>,

    /// 右上
    /// default: `e`
    pub up_right: Vec<String>,

    /// 左上
    /// default: `q`
    pub up_left: Vec<String>,

    /// 右下
    /// default: `v`
    pub down_right: Vec<String>,

    /// 左下
    /// default: `z`
    pub down_left: Vec<String>,

    /// 右ダッシュ
    /// default: `S-d`
    pub dash_right: Vec<String>,

    /// 左ダッシュ
    /// default: `S-a`
    pub dash_left: Vec<String>,

    /// 上ダッシュ
    /// default: `S-w`
    pub dash_up: Vec<String>,

    /// 下ダッシュ
    /// default: `S-s`
    pub dash_down: Vec<String>,

    /// 右上ダッシュ
    /// default: `S-e`
    pub dash_up_right: Vec<String>,

    /// 左上ダッシュ
    /// default: `S-q`
    pub dash_up_left: Vec<String>,

    /// 右下ダッシュ
    /// default: `S-v`
    pub dash_down_right: Vec<String>,

    /// 左下ダッシュ
    /// default: `S-z`
    pub dash_down_left: Vec<String>,
}

impl Default for KeyboardKeyBindings {
    ///
    ///
    /// q w e
    /// a _ d
    /// z s v
    ///
    /// dash: S-
    /// turn: C-
    fn default() -> Self {
        KeyboardKeyBindings {
            step: vec!["<space>".to_string()],
            turn_right: vec!["C-d".to_string()],
            turn_left: vec!["C-a".to_string()],
            turn_up: vec!["C-w".to_string()],
            turn_down: vec!["C-s".to_string()],
            turn_up_right: vec!["C-e".to_string()],
            turn_down_right: vec!["C-v".to_string()],
            turn_up_left: vec!["C-q".to_string()],
            turn_down_left: vec!["C-z".to_string()],
            right: vec!["d".to_string()],
            left: vec!["a".to_string()],
            up: vec!["w".to_string()],
            down: vec!["s".to_string()],
            up_right: vec!["e".to_string()],
            up_left: vec!["q".to_string()],
            down_right: vec!["v".to_string()],
            down_left: vec!["z".to_string()],
            dash_right: vec!["S-d".to_string()],
            dash_left: vec!["S-a".to_string()],
            dash_up: vec!["S-w".to_string()],
            dash_down: vec!["S-s".to_string()],
            dash_up_right: vec!["S-e".to_string()],
            dash_up_left: vec!["S-q".to_string()],
            dash_down_right: vec!["S-v".to_string()],
            dash_down_left: vec!["S-z".to_string()],
        }
    }
}
