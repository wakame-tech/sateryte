use bevy::prelude::*;
use bevy_crossterm::components::{Color, Position, Sprite, SpriteBundle, Style, StyleMap};

use crate::{
    dungeon_world::dungeon::Dungeon,
    player::components::{
        action::Action,
        event::PlayerActedEvent,
        guide_line::{GuideLine, IsGuideLine},
    },
};

/// ガイドラインをスポーンする
pub fn spawn_guide_lines(
    mut commands: Commands,
    dungeon: Option<Res<Dungeon>>,
    mut on_player_action: EventReader<PlayerActedEvent>,
    query: Query<Entity, With<IsGuideLine>>,
    mut sprites: ResMut<Assets<Sprite>>,
    mut stylemaps: ResMut<Assets<StyleMap>>,
) {
    for event in on_player_action.iter() {
        for entity in query.iter() {
            commands.entity(entity).despawn_recursive();
        }
        if let Action::Turn(dir) = &event.action {
            if let Some(ref dungeon) = dungeon {
                for point in dungeon.points_iter(event.pos, dir) {
                    let guide = sprites.add(Sprite::new("."));
                    let color = stylemaps.add(StyleMap::new(Style::with_fg(Color::Red), vec![]));

                    let sprite = SpriteBundle {
                        sprite: guide,
                        position: Position::new(point.x, point.y, 5),
                        stylemap: color,
                        ..Default::default()
                    };
                    let guideline = GuideLine::new(sprite);
                    commands.spawn_bundle(guideline);
                }
            }
        }
    }
}
