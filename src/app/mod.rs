pub mod game;
mod player_state;

pub struct GameManager {

}

#[derive(Default)]
pub struct PlayerState {
    hp: u64,
    max_hp: u64,
    power: u64
}