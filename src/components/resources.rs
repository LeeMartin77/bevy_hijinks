pub enum PlayState {
    Playing,
    Crashed
}

pub struct GameState {
    pub play_state: PlayState
}

impl GameState {
    pub fn new() -> GameState {
        GameState {
            play_state: PlayState::Playing
        }
    }
}