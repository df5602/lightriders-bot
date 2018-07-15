extern crate rand;

mod game_state;
mod parser;

use std::io;

use rand::Rng;

use game_state::GameState;
use parser::Input;

fn make_move(game_state: &GameState, bot_id: usize, rng: &mut rand::ThreadRng, _time_in_ms: usize) {
    let mut dir = rng.gen_range(0, 4);
    let mut cnt = 0;

    let (x, y) = match bot_id {
        0 => game_state.pos_player_0,
        1 => game_state.pos_player_1,
        _ => panic!("Unknown bot ID: {}", bot_id),
    };

    loop {
        match dir {
            0 => {
                if y > 0 && game_state.is_empty_at(x, y - 1) {
                    println!("up");
                    break;
                } else {
                    dir = (dir + 1) % 4;
                }
            }
            1 => {
                if y < game_state.height() - 1 && game_state.is_empty_at(x, y + 1) {
                    println!("down");
                    break;
                } else {
                    dir = (dir + 1) % 4;
                }
            }
            2 => {
                if x > 0 && game_state.is_empty_at(x - 1, y) {
                    println!("left");
                    break;
                } else {
                    dir = (dir + 1) % 4;
                }
            }
            3 => {
                if x < game_state.width() - 1 && game_state.is_empty_at(x + 1, y) {
                    println!("right");
                    break;
                } else {
                    dir = (dir + 1) % 4;
                }
            }
            _ => unreachable!(),
        }
        cnt += 1;

        if cnt > 3 {
            println!("pass");
            break;
        }
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
