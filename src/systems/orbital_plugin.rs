use bevy::prelude::*;
use crate::systems::*;

pub struct OrbitalPlugin;

impl Plugin for OrbitalPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup::setup.system())
            .add_system(input::player_input_system.system())
            .add_system(physics::thrust_system.system())
            .add_system(physics::gravity_system.system())
            .add_system(physics::velocity_system.system())
            .add_system(trail::trail_system.system());
    }
}