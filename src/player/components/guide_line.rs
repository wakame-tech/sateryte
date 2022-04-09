use bevy::prelude::*;
use bevy_crossterm::components::SpriteBundle;

#[derive(Component, Debug)]
pub struct IsGuideLine;

#[derive(Bundle)]
pub struct GuideLine {
    pub _marker: IsGuideLine,
    #[bundle]
    pub sprite: SpriteBundle,
}

impl GuideLine {
    pub fn new(sprite: SpriteBundle) -> Self {
        Self {
            _marker: IsGuideLine,
            sprite,
        }
    }
}
