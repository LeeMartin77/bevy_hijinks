use crate::components::resources::*;
use bevy::prelude::*;

use crate::components::entities;
use crate::components::physical_attributes;

const PLAYER_ACCELERATION_RATE: f32 = 5.0;
const PLAYER_TURN_RATE: f32 = 1.0;

pub fn player_input_system(
    time: Res<Time>,
    gamestate: Res<GameState>,
    keyboard_input: Res<Input<KeyCode>>,
    query: Query<(&entities::Player, &mut physical_attributes::Thrust, &mut Transform)>,
) {
    match gamestate.play_state {
        PlayState::Playing => handle_gameplay_input(time, keyboard_input, query),
        PlayState::Crashed => ()
    } 
}

fn handle_gameplay_input(time: Res<Time>, keyboard_input: Res<Input<KeyCode>>, mut query: Query<(&entities::Player, &mut physical_attributes::Thrust, &mut Transform)>){
    if let Ok((_player, mut thrust, mut transform)) = query.single_mut() {
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