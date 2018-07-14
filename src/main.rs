extern crate rand;

mod parser;

use std::io;

use rand::Rng;

use parser::Input;

fn make_move(rng: &mut rand::ThreadRng, _time_in_ms: usize) {
    let dir = rng.gen_range(0, 4);
    match dir {
        0 => println!("up"),
        1 => println!("down"),
        2 => println!("left"),
        3 => println!("right"),
        _ => unreachable!(),
    }
}

fn main() {
    let mut input = String::new();
    let stdin = io::stdin();

    let mut rng = rand::thread_rng();

    loop {
        input.clear();
        stdin.read_line(&mut input).unwrap();

        match parser::parse(&input) {
            Input::ActionMove(t) => make_move(&mut rng, t),
            s => eprintln!("Unhandled message: {:?}", s),
        }
    }
}
