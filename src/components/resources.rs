pub enum PlayState {
    StartMenu,
    Playing,
    Crashed
}

pub struct GameState {
    play_state: PlayState
}

impl GameState {
    pub fn new() -> GameState {
        GameState {
            play_state: PlayState::StartMenu
        }
    }
}