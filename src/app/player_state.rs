use crate::app::PlayerState;

/// Getter, Setter
impl PlayerState {
    pub fn get_hp(&self) -> &u64 {
        &self.hp
    }
}

/// public functions
impl PlayerState {
    pub fn take_damage(&mut self, damage: u64) {
        if damage > self.hp {
            self.hp = 0;
            return
        }
        self.hp -= damage;
    }

    pub fn take_heal(&mut self, heal: u64) {
        self.hp = u64::max(self.max_hp, self.hp + heal);
    }
}