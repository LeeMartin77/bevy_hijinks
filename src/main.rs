mod components;
mod systems;

use bevy::prelude::*;

use crate::components::*;
use crate::systems::*;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)))
        .add_startup_system(setup::setup.system())
        .add_system(input::player_input_system.system())
        .add_system(physics::thrust_system.system())
        .add_system(physics::gravity_system.system())
        .add_system(physics::velocity_system.system())
        .run();
}