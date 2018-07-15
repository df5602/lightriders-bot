extern crate rand;

mod game_state;
mod parser;
mod reachable;

use std::io;

use rand::Rng;

use game_state::GameState;
use parser::Input;

#[derive(Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    Pass,
}

fn make_move(game_state: &GameState, bot_id: usize, rng: &mut rand::ThreadRng, _time_in_ms: usize) {
    let (x, y) = match bot_id {
        0 => game_state.pos_player_0,
        1 => game_state.pos_player_1,
        _ => panic!("Unknown bot ID: {}", bot_id),
    };

    let mut moves: [(usize, Direction); 4] = [(0, Direction::Pass); 4];
    let mut n = 1;

    if y > 0 && game_state.is_empty_at(x, y - 1) {
        let r = reachable::count_reachable_cells(game_state, x, y - 1);
        if r > moves[0].0 {
            moves[0] = (r, Direction::Up);
            n = 1;
        } else if r == moves[0].0 {
            moves[n] = (r, Direction::Up);
            n += 1;
        }
    }

    if y < game_state.height() - 1 && game_state.is_empty_at(x, y + 1) {
        let r = reachable::count_reachable_cells(game_state, x, y + 1);
        if r > moves[0].0 {
            moves[0] = (r, Direction::Down);
            n = 1;
        } else if r == moves[0].0 {
            moves[n] = (r, Direction::Down);
            n += 1;
        }
    }

    if x > 0 && game_state.is_empty_at(x - 1, y) {
        let r = reachable::count_reachable_cells(game_state, x - 1, y);
        if r > moves[0].0 {
            moves[0] = (r, Direction::Left);
            n = 1;
        } else if r == moves[0].0 {
            moves[n] = (r, Direction::Left);
            n += 1;
        }
    }

    if x < game_state.width() - 1 && game_state.is_empty_at(x + 1, y) {
        let r = reachable::count_reachable_cells(game_state, x + 1, y);
        if r > moves[0].0 {
            moves[0] = (r, Direction::Right);
            n = 1;
        } else if r == moves[0].0 {
            moves[n] = (r, Direction::Right);
            n += 1;
        }
    }

    let best_direction = if n == 1 {
        moves[0].1
    } else {
        let dir = rng.gen_range(0, n);
        moves[dir].1
    };

    match best_direction {
        Direction::Up => println!("up"),
        Direction::Down => println!("down"),
        Direction::Left => println!("left"),
        Direction::Right => println!("right"),
        Direction::Pass => println!("pass"),
    }
}

fn main() {
    let mut input = String::new();
    let stdin = io::stdin();

    let mut rng = rand::thread_rng();

    let mut game_state = None;
    let mut bot_id = None;

    loop {
        input.clear();
        stdin.read_line(&mut input).unwrap();

        match parser::parse(&input) {
            Input::YourBotId(id) => bot_id = Some(id),
            Input::GameField(state) => game_state = Some(state),
            Input::ActionMove(t) => {
                make_move(game_state.as_ref().unwrap(), bot_id.unwrap(), &mut rng, t)
            }
            s => eprintln!("Unhandled message: {:?}", s),
        }
    }
}
