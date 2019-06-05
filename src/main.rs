/* Bring into scope these crates/modules */
use std::{env, process};

/**
 * Rust looks for ./game.rs or game/mod.rs
 */
mod game;

/*
    MAIN FUNCTION -> program starts by `main`
*/
fn main() {
    /*
    `Vec<String>` is an array of strings whose ownership is in this function scope.
    `env::args().collect()` is the way to collect command line arguments.
    "A heap-allocated vector that is resizable at runtime.""
    */
    
    let input_difficulty = ["easy", "medium", "hard"];
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Not enough arguments!  Must be `rust_edge [easy, medium, hard]`");
        process::exit(1);
    } else if args.len() > 2 {
        println!("Too many arguments!  Must be `rust_edge [easy, medium, hard]`");
        process::exit(1);
    }

    let difficulty: &str = &args[1];

    if input_difficulty.contains(&difficulty) {
        let _game_loop = game::looper(difficulty);
    } else {
        println!("Invalid difficulty.  You inputted {:?}", difficulty);
        println!("Too many arguments!  Must be `rust_edge [easy, medium, hard]`");
    }
}
