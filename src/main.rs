mod objects;
mod systems;

use crate::systems::{player_update, setup};

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, player_update)
        .run();
}
