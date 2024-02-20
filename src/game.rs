use std::io;
use std::io::{stdin, Write};

pub struct GameState {
    is_over: bool,
    is_debug: bool,
}

struct Statement {
    command: String,
    arguments: Vec<String>,
}

pub fn start() -> GameState {
    println!("Game start.");

    GameState {
        is_over: false,
        is_debug: true,
    }
}

pub fn game_over(game_state: &mut GameState) {
    game_state.is_over = true;
}

pub fn game_loop(mut game_state: GameState) {
    loop {
        if game_state.is_over {
            break;
        }

        print!("> ");
        io::stdout().flush().expect("flush failed!");

        let mut _input = String::new();
        stdin().read_line(&mut _input).expect("_input error");

        match convert_to_statement(&_input) {
            Some(statement) => game_state = read_statement(statement, game_state),
            None => println!()
        };

        if game_state.is_over {
            break;
        }
    }
}

fn convert_to_statement(input: &String) -> Option<Statement> {
    let mut words = input
        .split_whitespace()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    
    if words.iter().count() == 0 {
        return None
    }

    Some(Statement {
        command: words.remove(0),
        arguments: words,
    })
}

fn read_statement(statement: Statement, mut game_state: GameState) -> GameState {

    // debug log
    println!("command: {}, args: {:?}", statement.command, statement.arguments);

    if statement.command.eq("exit") || statement.command.eq("quit") {
        println!("game end");
        game_over(&mut game_state);
    }

    game_state
}
