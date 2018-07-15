extern crate rand;

mod game_state;
mod parser;
mod reachable;

use std::io;

use game_state::{CellType, GameState};
use parser::Input;

enum Direction {
    Up,
    Down,
    Left,
    Right,
    Pass,
}

fn make_move(
    game_state: &GameState,
    bot_id: usize,
    _rng: &mut rand::ThreadRng,
    _time_in_ms: usize,
) {
    let (x, y) = match bot_id {
        0 => game_state.pos_player_0,
        1 => game_state.pos_player_1,
        _ => panic!("Unknown bot ID: {}", bot_id),
    };

    let mut max_reachability = 0;
    let mut best_direction = Direction::Pass;

    if y > 0 && game_state.is_empty_at(x, y - 1) {
        let r = reachable::count_reachable_cells(game_state, x, y - 1);
        if r > max_reachability {
            max_reachability = r;
            best_direction = Direction::Up;
        }
    }

    if y < game_state.height() - 1 && game_state.is_empty_at(x, y + 1) {
        let r = reachable::count_reachable_cells(game_state, x, y + 1);
        if r > max_reachability {
            max_reachability = r;
            best_direction = Direction::Down;
        }
    }

    if x > 0 && game_state.is_empty_at(x - 1, y) {
        let r = reachable::count_reachable_cells(game_state, x - 1, y);
        if r > max_reachability {
            max_reachability = r;
            best_direction = Direction::Left;
        }
    }

    if x < game_state.width() - 1 && game_state.is_empty_at(x + 1, y) {
        let r = reachable::count_reachable_cells(game_state, x + 1, y);
        if r > max_reachability {
            //max_reachability = r;
            best_direction = Direction::Right;
        }
    }

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
                make_move(game_state.as_ref().unwrap(), bot_id.unwrap(), &mut rng, t);

                // TODO: remove
                let id = bot_id.unwrap();
                let (x, y) = match id {
                    0 => game_state.as_ref().unwrap().pos_player_0,
                    1 => game_state.as_ref().unwrap().pos_player_1,
                    _ => panic!("&&&"),
                };
                game_state.as_mut().unwrap().field[y * 16 + x] = CellType::Empty;
                eprintln!(
                    "Reachable cells: {}",
                    reachable::count_reachable_cells(game_state.as_ref().unwrap(), x, y)
                );
            }
            s => eprintln!("Unhandled message: {:?}", s),
        }
    }
}
