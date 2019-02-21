use serde_json::Value;
use std::{env, io};

// NOTE on TUI
// Maybe use TUI_RS?  https://github.com/fdehau/tui-rs
// Another one... https://github.com/gyscos/Cursive
// and another one...https://github.com/redox-os/termion
// They are all pretty even.

/*
`const`s are variable constants.
`&str` are references to the String wherever it is.
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
    /* Just a reference to the 1 index. */
    let difficulty = &args[1];

    /* Set environment variables for the program.*/
    env::set_var(CONFIG, GAME_CONFIG_STRING);
    env::set_var(USER_DIFFICULTY, difficulty);

    /*
    Fetch an environment variable and deserialize the configuration object.

    `env.var(CONFIG) returns a Result<T, E> type.

    NOTE*** To handle Result types, the `unwrap` function is used.  One could
    have used another way like `match` or `unwrap_or_else`

    https://doc.rust-lang.org/std/result/enum.Result.html
    */
    let retrieved_config = env::var(CONFIG).unwrap();
    let game_config: Value = serde_json::from_str(&retrieved_config).unwrap();

    println!("{}", game_config[difficulty]);

    let _game_loop = looper();

    /* Return a tuple for the `io::Result` of this function. */
    Ok(())
}

/* Immitate a CLI program */
fn looper() -> io::Result<()> {
    /* String ownership in this function scope.  It'll be changed by the STDIN */
    let mut user_input = String::new();

    loop {
        /*
        Locks this handle and reads a line of user_input into the specified buffer.
        See the above note about the `unwrap` function.
        */
        io::stdin().read_line(&mut user_input).unwrap();
        println!("You typed: {}", user_input.trim());

        /* Overwrite previous STDIN */
        user_input.clear();

        /* If "exit" is typed then stop the program. */
        if user_input == "exit\n" {
            break;
        }
    }

    /* Return a tuple for the `io::Result` of this function. */
    Ok(())
}
