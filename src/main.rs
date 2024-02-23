use crate::app::GameManager;
use crate::core::Inputter;

mod core;
mod app;

fn main() {
    let mut inputter = Inputter::build();
    inputter.add_performer(GameManager::new());
    inputter.run();
}
