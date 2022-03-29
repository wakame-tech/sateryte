use bevy::prelude::*;

use crate::player::{
    action_listener::world_action_listener, actions::Action, spawn::world_spawn_player,
};

use self::map::generate_map;

pub mod dungeon;
pub mod generator;
pub mod item;
pub mod map;
pub mod region;
pub mod tile;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(generate_map.label("map"))
            .add_system(world_spawn_player.after("map"))
            .add_system(world_action_listener);
    }
}
