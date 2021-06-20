mod components;
mod systems;

use bevy::prelude::*;

use crate::components::*;
use crate::systems::*;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)))
        .add_startup_system(setup.system())
        .add_system(player_input_system.system())
        .add_system(physics::thrust_system.system())
        .add_system(physics::gravity_system.system())
        .add_system(physics::velocity_system.system())
        .run();
}

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>
) {

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    let player_radius = 10.0;
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(Color::rgb(140.0, 140.0, 140.0).into()),
            transform: Transform::from_xyz(0.0, 150.0, 0.0),
            sprite: Sprite::new(Vec2::new(player_radius, player_radius)),
            ..Default::default()
        })
        .insert(entities::Player {})
        .insert(physical_attributes::Thrust {
            thrust: 0.0,
            facing: 0.0,
        })
        .insert(physical_attributes::Velocity {
            velocity: Vec3::new(150.0, 0.0, 0.0),
        })
        .insert(physical_attributes::Gravity::Movable(physical_attributes::MassRadius {
            radius: player_radius,
            mass: 0.001
        }));
        
    let planet_radius = 100.0;
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(Color::rgb(0.0, 0.0, 0.0).into()),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            sprite: Sprite::new(Vec2::new(planet_radius, planet_radius)),
            ..Default::default()
        })
        .insert(entities::Planet {
        })
        .insert(physical_attributes::Gravity::Immovable(physical_attributes::MassRadius {
            radius: planet_radius,
            mass: 500.0 * (10f32).powi(14)
        }));
}


const PLAYER_ACCELERATION_RATE: f32 = 50.0;
const PLAYER_TURN_RATE: f32 = 10.0;

fn player_input_system(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&entities::Player, &mut physical_attributes::Thrust)>,
) {
    if let Ok((_player, mut thrust)) = query.single_mut() {
        if keyboard_input.pressed(KeyCode::Left) {
            thrust.facing -= PLAYER_TURN_RATE * time.delta_seconds();
        }

        if keyboard_input.pressed(KeyCode::Right) {
            thrust.facing += PLAYER_TURN_RATE * time.delta_seconds();
        }

        if keyboard_input.pressed(KeyCode::Up) {
            thrust.thrust += PLAYER_ACCELERATION_RATE * time.delta_seconds();
        }

        if keyboard_input.pressed(KeyCode::Down) {
            thrust.thrust -= PLAYER_ACCELERATION_RATE * time.delta_seconds();
            
        }
        if thrust.thrust < 0.0 {
            thrust.thrust = 0.0;
        }
    }
}