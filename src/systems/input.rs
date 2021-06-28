use crate::systems::setup::starting_transform;
use crate::systems::setup::starting_velocity;
use crate::components::resources::*;
use bevy::prelude::*;

use crate::components::entities;
use crate::components::physical_attributes;

const PLAYER_ACCELERATION_RATE: f32 = 5.0;
const PLAYER_TURN_RATE: f32 = 1.0;

pub fn player_input_system(
    time: Res<Time>,
    gamestate: ResMut<GameState>,
    keyboard_input: Res<Input<KeyCode>>,
    query: Query<(&entities::Player, &mut physical_attributes::Velocity, &mut physical_attributes::Thrust, &mut Transform)>,
) {
    match gamestate.play_state {
        PlayState::Playing => handle_gameplay_input(time, keyboard_input, query),
        PlayState::Crashed => handle_crashed_input(keyboard_input, query, gamestate)
    } 
}

fn handle_gameplay_input(time: Res<Time>, keyboard_input: Res<Input<KeyCode>>, mut query: Query<(&entities::Player, &mut physical_attributes::Velocity, &mut physical_attributes::Thrust, &mut Transform)>){
    if let Ok((_player, _velocity, mut thrust, mut transform)) = query.single_mut() {
        let mut rotate: bool = false;
        if keyboard_input.pressed(KeyCode::Left) {
            thrust.facing -= PLAYER_TURN_RATE * time.delta_seconds();
            rotate = true;
        }
    
        if keyboard_input.pressed(KeyCode::Right) {
            thrust.facing += PLAYER_TURN_RATE * time.delta_seconds();
            rotate = true;
        }
        if rotate {
            transform.rotation = Quat::from_rotation_z(-thrust.facing);
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
        if thrust.thrust > 100.0 {
            thrust.thrust = 100.0;
        }
    }
}

fn handle_crashed_input(keyboard_input: Res<Input<KeyCode>>, mut query: Query<(&entities::Player, &mut physical_attributes::Velocity, &mut physical_attributes::Thrust, &mut Transform)>, mut gamestate: ResMut<GameState>){
    if keyboard_input.pressed(KeyCode::R) {
        if let Ok((_player, mut velocity, mut thrust, mut transform)) = query.single_mut() {
            thrust.thrust = 0.0;
            thrust.facing = 0.0;
            velocity.velocity = starting_velocity().velocity;
            velocity.crashed = false;
            transform.rotation = starting_transform().rotation;
            transform.translation = starting_transform().translation;
            gamestate.play_state = PlayState::Playing;
        }
    }
}