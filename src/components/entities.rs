use bevy::math::Vec2;

pub struct Player {

}

pub struct PositionHistory {
    pub history_interval: f32,
    pub history_delta: f32,
    pub history_length: u8,
    pub history: Vec<Vec2>,
    pub trail_entity: Option<bevy::prelude::Entity>

}

impl PositionHistory {
    pub fn new(history_interval: f32, history_length: u8, starting_position: Vec2) -> PositionHistory {
        PositionHistory {
            history_interval,
            history_delta: 0.0,
            history_length: history_length,
            history: vec![starting_position; history_length.into()],
            trail_entity: None
        }
    }
}

pub enum Planet {
    Starting
}