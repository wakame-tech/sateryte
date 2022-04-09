use bevy::{asset::HandleId, prelude::*};
use bevy_crossterm::components::{Color, Colors, Position, Sprite, SpriteBundle, StyleMap};

use crate::message::components::status_bar::{StatusBar, StatusBarOptions, StatusBarUpdateEvent};

pub fn status_bar_listener(
    mut commands: Commands,
    mut status_bar: ResMut<StatusBar>,
    mut reader: EventReader<StatusBarUpdateEvent>,
    mut sprites: ResMut<Assets<Sprite>>,
    mut stylemaps: ResMut<Assets<StyleMap>>,
    options: Res<StatusBarOptions>,
) {
    for event in reader.iter() {
        status_bar
            .props
            .insert(event.key.clone(), event.value.clone());
        let new_sprite = status_bar.to_sprite();

        // handleがあれば更新, なければ作成
        // TODO: Resourceで代替できないか
        status_bar.handle = if let Some(handle) = status_bar.handle {
            sprites.set_untracked(handle, new_sprite);
            Some(handle)
        } else {
            let sprite = sprites.add(new_sprite);
            let handle: HandleId = sprite.clone().into();
            let color = stylemaps.add(StyleMap::with_colors(Colors::new(
                Color::White,
                Color::Cyan,
            )));
            let bundle = SpriteBundle {
                sprite,
                position: Position::new(options.area.pos.x, options.area.pos.y, 4),
                stylemap: color,
                ..Default::default()
            };
            commands.spawn_bundle(bundle);
            Some(handle)
        }
    }
}
