#[derive(Debug)]
pub struct SateryteOptions {
    pub is_debug: bool,
}

impl Default for SateryteOptions {
    fn default() -> Self {
        Self { is_debug: false }
    }
}
