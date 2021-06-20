use bevy::prelude::*;

use crate::components::*;

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let player_radius = 1.0;
    commands
        .spawn_bundle(PbrBundle {
            material: materials.add(StandardMaterial { 
                base_color: Color::rgb(1.0, 0.0, 0.0).into(),
                metallic: 0.9,
                roughness: 0.3,
                ..Default::default()
            }),
            transform: Transform::from_xyz(0.0, 15.0, 0.0),
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
            velocity: Vec3::new(15.0, 0.0, 0.0),
        })
        .insert(physical_attributes::Gravity::Movable(physical_attributes::MassRadius {
            radius: player_radius,
            mass: 0.0001
        }));
        
    let planet_radius = 10.0;
    commands
        .spawn_bundle(PbrBundle {
            material: materials.add(StandardMaterial { 
                base_color: Color::rgb(0.3, 0.3, 0.3).into(),
                metallic: 0.1,
                roughness: 0.7,
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
            mass: 5.0 * (10f32).powi(13)
        }));

    let mut camera = OrthographicCameraBundle::new_3d();
    camera.orthographic_projection.scale = 50.0;
    camera.transform = Transform::from_xyz(0.0, 0.0, 25.0).looking_at(Vec3::ZERO, Vec3::Y);

    commands.spawn_bundle(camera);
    
    commands.spawn_bundle(LightBundle {
        light: Light {
            range: 200.0,
            intensity: 2000.0,
            fov: f32::to_radians(360.0),
            ..Default::default()
        },
        transform: Transform::from_xyz(-50.0, 40.0, 20.0),
        ..Default::default()
    });
}