mod objects;
mod systems;

use crate::systems::{player_update, setup, handle_inputs};

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, player_update)
        .add_systems(Update, handle_inputs)
        .run();
}
