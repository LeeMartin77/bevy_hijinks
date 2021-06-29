use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use crate::components::entities;

pub fn trail_system(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(&mut entities::PositionHistory, &Transform)>,
) {
    for (mut position_history, transform) in query.iter_mut() {
        position_history.history_delta += time.delta_seconds();
        if position_history.history_delta > position_history.history_interval {
            position_history.history_delta -= position_history.history_interval;
            let mut new_history = Vec::new();
            new_history.push(Vec2::new(transform.translation.x, transform.translation.y));
            let mut iterator: usize = 0;
            for item in &position_history.history {
                if iterator < position_history.history_length.into() {
                    new_history.push(*item)
                }
                iterator += 1;
            }
            position_history.history = new_history;
            let trail_shape = shapes::Polygon {
                closed: false,
                points: position_history.history.clone()
            };
            if let Some(trail_entity) = position_history.trail_entity {
                commands.entity(trail_entity).despawn();
            }
            position_history.trail_entity = Some(commands.spawn_bundle(
                GeometryBuilder::build_as(
                    &trail_shape,
                    ShapeColors::new(Color::rgb(1.0, 0.0, 0.0)),
                    DrawMode::Stroke(StrokeOptions::default().with_line_width(0.2)),
                    Transform::from_xyz(0.0, 0.0, 0.0),
                )
            ).id());
        }
    }
}