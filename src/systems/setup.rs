use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use crate::components::*;

pub enum UiTextElements {
    RestartText
}

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    commands.add_player();

    let planet_radius = 10.0;
    let planet_density = 12000000000.0;

    commands.add_starting_planet(planet_radius, planet_density, 0.0, 0.0);

    commands.add_camera();

    commands.spawn_bundle(UiCameraBundle::default());

    commands
    .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position: Rect {
                    bottom: Val::Px(5.0),
                    right: Val::Px(15.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            text: Text::with_section(
                "Press R To Restart",
                TextStyle {
                    font: asset_server.load("fonts/ShareTechMono-Regular.ttf"),
                    font_size: 20.0,
                    color: Color::WHITE,
                },
                TextAlignment {
                    horizontal: HorizontalAlign::Center,
                    ..Default::default()
                },
            ),
            ..Default::default()
        })
        .insert(UiTextElements::RestartText);
}

//TODO: This is hideous, find a better way of doing this. Resource?
pub fn starting_transform() -> Transform {
    Transform::from_xyz(0.0, 15.0, 0.0)
}

pub fn starting_velocity() -> physical_attributes::Velocity {
    physical_attributes::Velocity::new(15.0, 0.0)
}

trait SetupExtensions {
    fn add_camera(&mut self);
    fn add_player(&mut self);
    fn add_starting_planet(&mut self, planet_radius: f32, planet_density: f32, x: f32, y: f32);
}

impl SetupExtensions for Commands<'_> {
    fn add_camera(&mut self) {
        let mut camera = OrthographicCameraBundle::new_2d();
        camera.orthographic_projection.scale = 0.2;
        camera.transform = Transform::from_xyz(0.0, 0.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y);
    
        self.spawn_bundle(camera);
    }
    fn add_player(&mut self) {
        let player_radius = 1.0;
    
        let player_shape = shapes::Polygon {
            closed: true,
            points: vec!(Vec2::new(0.0, 1.0), Vec2::new(0.75, 0.5), Vec2::new(1.0, -1.0), Vec2::new(0.0, -0.75), Vec2::new(-1.0, -1.0), Vec2::new(-0.75, 0.5))
        };
        self
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
    }

    fn add_starting_planet(&mut self, planet_radius: f32, planet_density: f32, x: f32, y: f32) {
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
            .insert(entities::Planet::Starting)
            .insert(physical_attributes::Gravity::Immovable(physical_attributes::MassRadius::from_density(planet_density, planet_radius)));
    }
}




