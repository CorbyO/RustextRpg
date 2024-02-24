use std::ops::Index;
use crate::core::Statement;

/// constructor, getter, setter
impl Statement {
    pub fn new(mut raw_command: String) -> Option<Statement> {
        let mut words = raw_command
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
    
    pub fn get_argument(&self, index : usize) -> &str {
        if self.arguments.iter().count() <= index {
            return "";
        }

        self.arguments.index(index).as_str()
    }
}