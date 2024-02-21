use crate::inputter::{Inputter, Statement};

mod inputter;

struct Game {}

impl Game {
    pub fn new() -> Game {
        Game {}
    }
}

impl inputter::Performable for Game {
    fn run(&self, statement: &Statement) -> bool {
        if statement.command.eq("exit") || statement.command.eq("quit") {
            println!("game end");
            return false;
        }

        return true;
    }
}

fn main() {
    let mut inputter = Inputter::build();
    inputter.add_performer(Game::new());
    inputter.run();
}
