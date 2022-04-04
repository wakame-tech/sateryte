use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Turn(pub u32);

impl Turn {
    pub fn new() -> Self {
        Self(0)
    }

    pub fn increment(&mut self) {
        self.0 += 1;
    }
}
