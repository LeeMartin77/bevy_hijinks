use bevy::utils::Uuid;
use bevy::prelude::*;
use crate::systems::*;
use crate::components::resources::GameState;
use crate::systems::planet_generation::PlanetGenerationData;

pub struct OrbitalPlugin;

impl Plugin for OrbitalPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .insert_resource(GameState::new())
            .insert_resource(PlanetGenerationData::new(Uuid::new_v4().to_string()))
            .add_startup_system(setup::setup.system())
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(planet_generation::when_to_generate.system())
                    .with_system(planet_generation::planet_generation_system.system()),
            )
            .add_system(ui::ui_system.system())
            .add_system(gamestate::gamestate_system.system())
            .add_system(input::player_input_system.system())
            .add_system(physics::thrust_system.system())
            .add_system(physics::gravity_system.system())
            .add_system(physics::velocity_system.system())
            .add_system(trail::trail_system.system());
    }
}