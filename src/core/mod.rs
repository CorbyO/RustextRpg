pub mod inputter;

pub struct Statement {
    pub command: String,
    pub arguments: Vec<String>,
}

pub trait Performable {
    fn run(&self, statement: &Statement) -> bool;
}

pub struct Inputter {
    is_over: bool,
    performances: Vec<Box<dyn Performable>>,
}
