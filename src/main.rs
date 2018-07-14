extern crate rand;

use std::io;

use rand::Rng;

fn main() {
    let mut input = String::new();
    let stdin = io::stdin();

    let mut rng = rand::thread_rng();

    loop {
        input.clear();
        stdin.read_line(&mut input).unwrap();
        eprintln!("{}", input);

        if input.starts_with("action move") {
            let dir = rng.gen_range(0, 4);
            match dir {
                0 => println!("up"),
                1 => println!("down"),
                2 => println!("left"),
                3 => println!("right"),
                _ => unreachable!(),
            }
        }
    }
}
