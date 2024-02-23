use crate::app::GameManager;
use crate::core::{Performable, Statement};

impl GameManager {
    pub fn new() -> GameManager {
        GameManager {}
    }
}

impl Performable for GameManager {
    fn run(&self, statement: &Statement) -> bool {
        if statement.command.eq("exit") || statement.command.eq("quit") {
            println!("game end");
            return false;
        }

        return true;
    }
}