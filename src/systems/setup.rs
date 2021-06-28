use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use crate::components::*;

pub fn setup(
    mut commands: Commands
) {
    commands = add_player(commands);

    let planet_radius = 10.0;
    let planet_density = 12000000000.0;

    commands = add_starting_planet(commands, planet_radius, planet_density, 0.0, 0.0);

    add_camera(commands);
}

//TODO: This is hideous, find a better way of doing this. Resource?
pub fn starting_transform() -> Transform {
    Transform::from_xyz(0.0, 15.0, 0.0)
}

pub fn starting_velocity() -> physical_attributes::Velocity {
    physical_attributes::Velocity::new(15.0, 0.0)
}

fn add_camera(mut commands: Commands) -> Commands {
    let mut camera = OrthographicCameraBundle::new_2d();
    camera.orthographic_projection.scale = 0.2;
    camera.transform = Transform::from_xyz(0.0, 0.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y);

    commands.spawn_bundle(camera);
    commands
}

fn add_player(mut commands: Commands) -> Commands {
    let player_radius = 1.0;

    let player_shape = shapes::Polygon {
        closed: true,
        points: vec!(Vec2::new(0.0, 1.0), Vec2::new(0.75, 0.5), Vec2::new(1.0, -1.0), Vec2::new(0.0, -0.75), Vec2::new(-1.0, -1.0), Vec2::new(-0.75, 0.5))
    };
    commands
        .spawn_bundle(
            GeometryBuilder::build_as(
                &player_shape,
                ShapeColors::new(Color::BLACK),
                DrawMode::Fill(FillOptions::default()),
                starting_transform(),
            )
        )
        .insert(entities::Player {})
        .insert(entities::PositionHistory::new(0.02, 100, Vec2::new(0.0, 15.0)))
        .insert(physical_attributes::Thrust {
            thrust: 0.0,
            facing: 0.0,
        })
        .insert(starting_velocity())
        .insert(physical_attributes::Gravity::Movable(physical_attributes::MassRadius {
            radius: player_radius,
            mass: 0.0001
        }));
    commands
}


fn add_starting_planet(mut commands: Commands, planet_radius: f32, planet_density: f32, x: f32, y: f32) -> Commands {
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
        .insert(entities::Planet::Starting)
        .insert(physical_attributes::Gravity::Immovable(physical_attributes::MassRadius::from_density(planet_density, planet_radius)));
    commands
}