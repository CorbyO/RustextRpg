use crate::app::{GameManager, PlayerState};
use crate::core::{Performable, Statement};

impl GameManager {
    pub fn new() -> GameManager {
        GameManager {
            player_state: PlayerState::new(50, 10),
        }
    }
}

impl Performable for GameManager {
    fn run(&mut self, statement: &Statement) -> bool {

        let mut is_not_stop = true;

        match statement.command.as_str() {
            "exit" | "quit" => {
                println!("game end");
                is_not_stop = false;
            },
            "test" => match statement.get_argument(0) {
                "damage" => {
                    self.player_state.take_damage(10);
                    println!("hp: {}", self.player_state.get_hp())
                }
                "heal" => {
                    self.player_state.take_heal(10);
                    println!("hp: {}", self.player_state.get_hp())
                }
                "" => println!("Test command need more arguments."),
                arg => println!("Test command not support '{}' argument.", arg),
            },
            _ => {}
        }

        return is_not_stop;
    }
}