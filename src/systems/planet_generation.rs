use bevy::ecs::schedule::ShouldRun;
use bevy::prelude::*;
use crate::components::entities::Planet;
use crate::components::physical_attributes::*;
use bevy_prototype_lyon::prelude::*;
use rand::prelude::*;
use rand_seeder::Seeder;
use rand_pcg::Pcg64;

const CHUNK_SIZE: f32 = 40.0;
// We want some margin between chunks
const HALF_CHUNK_ACTIVE: f32 = 10.0;

const GENERATION_RANGE: i32 = 10;

pub struct PlanetGenerationData {
    world_rng: Pcg64,
    should_generate: bool,
    chunks_generated: Vec<Vec2>
}

impl PlanetGenerationData {
    pub fn new(world_seed: String) -> PlanetGenerationData{
        PlanetGenerationData {
            world_rng: Seeder::from(world_seed).make_rng(),
            should_generate: true,
            // 0,0 will already have a planet, and it's the center chunk
            chunks_generated: vec!(Vec2::new(0.0, 0.0))
        }
    }

    pub fn chunk_already_generated(&self, chunk_center: Vec2) -> bool {
        self.chunks_generated.iter().any(|&chnk| chnk.x == chunk_center.x && chnk.y == chunk_center.y)
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
    mut commands: Commands,
    mut planet_generation_data: ResMut<PlanetGenerationData>,
    //mut set: QuerySet<(
    //    Query<()>,
    //    Query<()>
    //)>
) {
    //For now, we're going to do this from 0,0, later we'll want to do this dynamically from player position.
    let generation_center = Vec2::new(0.0, 0.0);

    //for each "chunk", + and - in both x and y of generation range, where not in list

    let x_int = generation_center.x as i32;
    let y_int = generation_center.y as i32;

    let planet_radius = 10.0;
    let planet_density = 12000000000.0;

    for x in (x_int - GENERATION_RANGE)..(x_int + GENERATION_RANGE) {
        for y in (y_int - GENERATION_RANGE)..(y_int + GENERATION_RANGE) {
            let chunk_center = Vec2::new(x as f32 * CHUNK_SIZE, y as f32 * CHUNK_SIZE);
            if planet_generation_data.chunk_already_generated(chunk_center) {
                continue;
            }
            //generate planet 25% of the time
            let generate: f32 = planet_generation_data.world_rng.gen();
            if generate > 0.75 {

                let x_deviation: f32 = planet_generation_data.world_rng.gen_range(-1.0..1.0);
                let y_deviation: f32 = planet_generation_data.world_rng.gen_range(-1.0..1.0);
                
                commands.add_planet(
                    planet_radius,
                    planet_density,
                    chunk_center.x + (x_deviation * HALF_CHUNK_ACTIVE), 
                    chunk_center.y + (y_deviation * HALF_CHUNK_ACTIVE)
                );
            }

            planet_generation_data.chunks_generated.push(chunk_center);
        }
    }


}

trait PlanetGenerationCommand {
    fn add_planet(&mut self, planet_radius: f32, planet_density: f32, x: f32, y: f32);
}

impl PlanetGenerationCommand for Commands<'_> {
    fn add_planet(&mut self, planet_radius: f32, planet_density: f32, x: f32, y: f32) {
        let planet_circle = shapes::Circle {
            radius: planet_radius,
            ..shapes::Circle::default()
        };
        self
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
}

