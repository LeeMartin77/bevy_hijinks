use bevy::ecs::schedule::ShouldRun;
use bevy::prelude::*;
use crate::components::entities::Planet;
use crate::components::physical_attributes::*;
use bevy_prototype_lyon::prelude::*;
use rand::prelude::*;
use rand_seeder::Seeder;
use rand_pcg::Pcg64;


//Later, we're going to introduce chunks properly - but lets get repeatable random first.
const CHUNK_SIZE: f32 = 20.0;

pub struct PlanetGenerationData {
    world_rng: Pcg64,
    should_generate: bool
}

impl PlanetGenerationData {
    pub fn new(world_seed: String) -> PlanetGenerationData{
        PlanetGenerationData {
            world_rng: Seeder::from(world_seed).make_rng(),
            should_generate: true
        }
    }
}

pub fn when_to_generate(
    mut generated: ResMut<PlanetGenerationData>
) -> ShouldRun {
    if generated.should_generate {
        generated.should_generate = false;
        return ShouldRun::Yes;
    }
    ShouldRun::No
}

pub fn planet_generation_system(
    commands: Commands,
    mut planet_generation_data: ResMut<PlanetGenerationData>,
    //mut set: QuerySet<(
    //    Query<()>,
    //    Query<()>
    //)>
) {

    let planet_radius = 10.0;
    let planet_density = 12000000000.0;

    let x_deviation: f32 = planet_generation_data.world_rng.gen();
    let y_deviation: f32 = planet_generation_data.world_rng.gen();

    add_planet(commands, planet_radius, planet_density, 30.0 + (x_deviation* CHUNK_SIZE), y_deviation * CHUNK_SIZE);
}

fn add_planet(mut commands: Commands, planet_radius: f32, planet_density: f32, x: f32, y: f32) {
    let planet_circle = shapes::Circle {
        radius: planet_radius,
        ..shapes::Circle::default()
    };
    commands
        .spawn_bundle(
            GeometryBuilder::build_as(
                &planet_circle,
                ShapeColors::new(Color::BLACK),
                DrawMode::Fill(FillOptions::default()),
                Transform::from_xyz(x, y, 0.0),
            )
        )
        .insert(Planet::Starting)
        .insert(Gravity::Immovable(MassRadius::from_density(planet_density, planet_radius)));
}