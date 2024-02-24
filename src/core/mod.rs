mod inputter;
mod statement;

pub struct Statement {
    pub command: String,
    arguments: Vec<String>,
}

pub trait Performable {
    fn run(&mut self, statement: &Statement) -> bool;
}

pub struct Inputter {
    is_over: bool,
    performances: Vec<Box<dyn Performable>>,
}
