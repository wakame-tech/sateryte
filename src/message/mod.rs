use bevy::prelude::*;

use self::message_window::{draw_message_window, message_listener, setup_message_window};

pub mod message_window;
#[derive(Component)]
pub struct MessageWindow {
    pub messages: Vec<Message>,
}

#[derive(Component, Clone)]
pub struct Message {
    pub text: String,
}

pub struct MessagePlugin;

impl Plugin for MessagePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<Message>()
            .add_startup_system(setup_message_window)
            .add_system(message_listener)
            .add_system(draw_message_window);
    }
}
