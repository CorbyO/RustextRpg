use std::io;
use std::io::{stdin, Write};
use crate::core::{Inputter, Performable, Statement};

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

            match Statement::new(_input) {
                Some(statement) => self.read_statement(&statement),
                None => println!()
            };
        }
    }

    fn read_statement(&mut self, statement: &Statement) {
        // debug log
        println!("command: {}, args: {:?}", statement.command, statement.arguments);

        for performance in self.performances.iter_mut() {
            if !performance.run(statement)
            {
                self.stop();
                break;
            }
        }
    }

    pub fn add_performer<T: Performable + 'static>(&mut self, performable: T) {
        self.performances.push(Box::new(performable));
    }
}