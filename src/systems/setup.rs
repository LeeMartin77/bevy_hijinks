use bevy::prelude::*;

use crate::components::*;

pub fn setup(
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