use std::io;
use std::io::{stdin, Write};

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

impl Inputter {
    pub fn build() -> Inputter {
        Inputter {
            is_over: false,
            performances: Vec::new(),
        }
    }

    fn stop(&mut self) {
        self.is_over = true;
    }

    pub fn run(&mut self) {
        loop {
            if self.is_over {
                break;
            }

            print!("> ");
            io::stdout().flush().expect("flush failed!");

            let mut _input = String::new();
            stdin().read_line(&mut _input).expect("_input error");

            match self.convert_to_statement(&_input) {
                Some(statement) => self.read_statement(statement),
                None => println!()
            };

            if self.is_over {
                break;
            }
        }
    }

    fn convert_to_statement(&mut self, input: &String) -> Option<Statement> {
        let mut words = input
            .split_whitespace()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();

        if words.iter().count() == 0 {
            return None;
        }

        Some(Statement {
            command: words.remove(0),
            arguments: words,
        })
    }

    fn read_statement(&mut self, statement: Statement) {

        // debug log
        println!("command: {}, args: {:?}", statement.command, statement.arguments);

        for performance in self.performances.iter() {
            if !performance.run(&statement)
            {
                self.stop();
                break;
            }
        }
    }

    pub fn add_performer(&mut self, performable: impl Performable + 'static) {
        self.performances.push(Box::new(performable));
    }
}
