use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use crate::components::*;

pub fn setup(
    mut commands: Commands
) {
    //I'm 100% sure this can be gotten in data, but lets take the path of least resistance right now
    let player_radius = 1.0;

    let player_circle = shapes::Circle {
        radius: player_radius,
        ..shapes::Circle::default()
    };
    commands
        .spawn_bundle(
            GeometryBuilder::build_as(
                &player_circle,
                ShapeColors::new(Color::TEAL),
                DrawMode::Fill(FillOptions::default()),
                Transform::from_xyz(0.0, 15.0, 0.0),
            )
        )
        .insert(entities::Player { })
        .insert(physical_attributes::Thrust {
            thrust: 0.0,
            facing: 0.0,
        })
        .insert(physical_attributes::Velocity {
            velocity: Vec3::new(15.0, 0.0, 0.0),
        })
        .insert(physical_attributes::Gravity::Movable(physical_attributes::MassRadius {
            radius: player_radius,
            mass: 0.0001
        }));
        
    let planet_radius = 10.0;

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
                Transform::from_xyz(0.0, 0.0, 0.0),
            )
        )
        .insert(entities::Planet { })
        .insert(physical_attributes::Gravity::Immovable(physical_attributes::MassRadius {
            radius: planet_radius,
            mass: 5.0 * (10f32).powi(13)
        }));

    commands
        .spawn_bundle(
            GeometryBuilder::build_as(
                &planet_circle,
                ShapeColors::new(Color::BLACK),
                DrawMode::Fill(FillOptions::default()),
                Transform::from_xyz(45.0, 0.0, 0.0),
            )
        )
        .insert(entities::Planet { })
        .insert(physical_attributes::Gravity::Immovable(physical_attributes::MassRadius {
            radius: planet_radius,
            mass: 5.0 * (10f32).powi(13)
        }));

    let mut camera = OrthographicCameraBundle::new_2d();
    camera.orthographic_projection.scale = 0.1;
    camera.transform = Transform::from_xyz(0.0, 0.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y);

    commands.spawn_bundle(camera);
}