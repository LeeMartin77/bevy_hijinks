use bevy::prelude::*;
use crate::systems::*;
use crate::components::resources::GameState;

pub struct OrbitalPlugin;

impl Plugin for OrbitalPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .insert_resource(GameState::new())
            .add_startup_system(setup::setup.system())
            .add_system(gamestate::gamestate_system.system())
            .add_system(input::player_input_system.system())
            .add_system(physics::thrust_system.system())
            .add_system(physics::gravity_system.system())
            .add_system(physics::velocity_system.system())
            .add_system(trail::trail_system.system());
    }
}