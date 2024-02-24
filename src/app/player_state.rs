use std::ops::{Add, Sub};
use crate::app::PlayerState;

/// Getter, Setter
impl PlayerState {
    pub fn get_hp(&self) -> u32 {
        self.hp
    }

    pub fn is_dead(&self) -> bool {
        self.hp == 0
    }
}

/// public functions
impl PlayerState {
    pub fn new(hp: u32, power: u32) -> PlayerState {
        PlayerState {
            hp,
            max_hp: hp,
            power,
        }
    }

    pub fn take_damage(&mut self, damage: u32) {
        match self.hp.checked_sub(damage) {
            Some(x) => self.hp = x,
            None => self.hp = 0,
        }
    }

    pub fn take_heal(&mut self, heal: u32) {
        match self.hp.checked_add(heal) {
            Some(x) => self.hp = x,
            None => self.hp = u32::MAX
        }

        self.hp = self.hp.min(self.max_hp);
    }
}

#[cfg(test)]
mod tests {
    use crate::app::PlayerState;

    #[test]
    fn hp_test() {
        let mut player_state = PlayerState::new(50, 10);

        player_state.take_damage(5);
        assert_eq!(player_state.hp, 45);
        player_state.take_damage(50);
        assert_eq!(player_state.hp, 0);
        player_state.take_heal(10);
        assert_eq!(player_state.hp, 10);
        player_state.take_heal(100);
        assert_eq!(player_state.hp, 50);
    }
}