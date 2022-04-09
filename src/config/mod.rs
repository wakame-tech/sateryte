use crate::geo::size::Size;

#[derive(Debug)]
pub struct SateryteOptions {
    /// 画面サイズ
    pub size: Size,
    /// デバッグモードか
    pub is_debug: bool,
}

impl SateryteOptions {
    pub fn new(size: Size) -> Self {
        Self {
            size,
            is_debug: false,
        }
    }
}
