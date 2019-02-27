/* Bring into scope these crates/modules */
use std::{env, io, process};

/**
 * Rust looks for ./game.rs or game/mod.rs
 */
mod game;

/*
`const`s are variable constants.
`&str` are references to the String wherever it is and they are always a fixed
length and could never change.
*/
const CONFIG: &str = "GAME_CONFIG";
const USER_DIFFICULTY: &str = "DIFF";

/* This was a way to include the file as a string when the program is compiled. */
/* This is serializing it! */
const GAME_CONFIG_STRING: &str = include_str!("config.json");

/*
    MAIN FUNCTION -> program starts by `main`
*/
fn main() -> io::Result<()> {
    /*
    `Vec<String>` is an array of strings whose ownership is in this function scope.
    `env::args().collect()` is the way to collect commandline arguments.
    "A heap-allocated vector that is resizable at runtime.""
    */
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Not enough arguments!  Must be `cargo run [easy, medium, hard]`");
        process::exit(1);
    }

    /* Just a reference to the 1 index. */
    let difficulty = &args[1];

    /* Set environment variables for the program.*/
    env::set_var(CONFIG, GAME_CONFIG_STRING);
    env::set_var(USER_DIFFICULTY, difficulty);

    let _game_loop = game::looper();

    /* Return a tuple for the `io::Result` of this function. */
    Ok(())
}
