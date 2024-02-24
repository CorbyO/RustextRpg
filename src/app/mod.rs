mod game;
mod player_state;

pub struct GameManager {
    pub player_state : PlayerState,
}

#[derive(Default)]
pub struct PlayerState {
    hp: u32,
    max_hp: u32,
    power: u32
}