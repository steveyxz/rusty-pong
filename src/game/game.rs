use bevy::{
    DefaultPlugins,
    prelude::{App, Plugin},
};

use crate::ecs::systems::{initialize, keyboard_input, update_velocities};

pub struct PongGame;

impl Plugin for PongGame {
    fn build(&self, app: &mut App) {
        app.add_system(update_velocities).add_system(keyboard_input);
    }
}

impl PongGame {
    pub fn start() {
        App::new()
            .add_plugins(DefaultPlugins)
            .add_plugin(PongGame)
            .add_startup_system(initialize)
            .run();
    }
}
