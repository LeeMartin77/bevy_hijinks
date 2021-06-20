use bevy::prelude::*;

use crate::entities;
use crate::physical_attributes;

const PLAYER_ACCELERATION_RATE: f32 = 50.0;
const PLAYER_TURN_RATE: f32 = 10.0;

pub fn player_input_system(
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