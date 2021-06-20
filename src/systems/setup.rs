use bevy::prelude::*;

use crate::components::*;

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let player_radius = 10.0;
    commands
        .spawn_bundle(PbrBundle {
            material: materials.add(StandardMaterial { 
                base_color: Color::rgb(140.0, 140.0, 140.0).into(),
                ..Default::default()
            }),
            transform: Transform::from_xyz(0.0, 150.0, 0.0),
            mesh: meshes.add(Mesh::from(shape::Icosphere { 
                radius: player_radius, 
                subdivisions: 32, 
            })),
            ..Default::default()
        })
        .insert(entities::Player { })
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
        .spawn_bundle(PbrBundle {
            material: materials.add(StandardMaterial { 
                base_color: Color::rgb(0.0, 0.0, 0.0).into(),
                ..Default::default()
            }),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            mesh: meshes.add(Mesh::from(shape::Icosphere { 
                radius: planet_radius, 
                subdivisions: 32, 
            })),
            ..Default::default()
        })
        .insert(entities::Planet { })
        .insert(physical_attributes::Gravity::Immovable(physical_attributes::MassRadius {
            radius: planet_radius,
            mass: 500.0 * (10f32).powi(14)
        }));

    let mut camera = OrthographicCameraBundle::new_3d();
    camera.orthographic_projection.scale = 500.0;
    camera.transform = Transform::from_xyz(0.0, 0.0, 250.0).looking_at(Vec3::ZERO, Vec3::Y);

    commands.spawn_bundle(camera);
    
    commands.spawn_bundle(LightBundle {
        transform: Transform::from_xyz(0.0, 0.0, 700.0),
        ..Default::default()
    });
}