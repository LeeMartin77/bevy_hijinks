use bevy::prelude::*;
use crate::components::*;

pub fn gamestate_system(
    mut gamestate: ResMut<resources::GameState>,
    query: Query<(&entities::Player, &physical_attributes::Velocity)>,
) {
    if let Ok((_player, velocity)) = query.single() {
        if velocity.crashed {
            gamestate.play_state = resources::PlayState::Crashed
        }
    }
}