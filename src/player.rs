use bevy::prelude::*;
use bevy_crossterm::components::{Position, Sprite, SpriteBundle, StyleMap};

use super::input::actions::Action;
#[derive(Component)]
pub struct Player;

pub fn startup_player(
    mut commands: Commands,
    mut sprites: ResMut<Assets<Sprite>>,
    mut stylemaps: ResMut<Assets<bevy_crossterm::components::StyleMap>>,
) {
    let player = sprites.add(Sprite::new("@"));
    let color = stylemaps.add(StyleMap::default());

    commands
        .spawn_bundle(SpriteBundle {
            sprite: player,
            position: Position::with_xy(0, 0),
            stylemap: color,
            ..Default::default()
        })
        .insert(Player);
}

pub fn action_listener(
    mut reader: EventReader<Action>,
    mut query: Query<(&Player, &mut Position)>,
) {
    for action in reader.iter() {
        let (_, mut player) = query.single_mut();
        match action {
            Action::Right => {
                player.x += 1;
            }
            Action::Left => {
                player.x -= 1;
            }
            Action::Up => {
                player.y -= 1;
            }
            Action::Down => {
                player.y += 1;
            }
        }
    }
}
