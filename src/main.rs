mod game;

fn main() {
    let _game_state = game::start();
    game::game_loop(_game_state);
}
