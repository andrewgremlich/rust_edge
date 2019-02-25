/* Bring in io into scope of this file. */
use std::io;

/** See mod note in src/main.rs */
mod game_config;
mod game_string_constants;
mod map;

/* Immitate a CLI program */
pub fn looper() -> io::Result<()> {
    /*
    String ownership in this function scope.  It'll be changed by the STDIN.
    String datatype can be changed whenever.
    */
    let mut user_input = String::new();

    /* Fetch the game config from game ENV. */
    let game_config = game_config::fetch_config();

    /*
    The game_config, for some reason can't be easily passed through to the map
    function probably because of scopes and ownership of the IndivdiualConfig struct.
    So I'm passing the properties that I need from the game_config struct.
    */
    map::init_map(
        game_config.xLength,
        game_config.yLength,
        game_config.xGoal,
        game_config.yGoal,
        game_config.numDangers,
        game_config.showMap,
    );

    loop {
        /*
        Locks this handle and reads a line of user_input into the specified buffer.
        See the above note about the `unwrap` function.
        */
        io::stdin().read_line(&mut user_input).unwrap();

        /* Match the user_input to command. */
        /* .trim() helps user_input be a reference for the match to work. */
        match user_input.trim() {
            "m" => println!("you want the map."),
            "a" => println!("you want to change positions in the tomb."),
            "s" => println!("mark suspected danger."),
            "r" => println!("remind of nearby danger"),
            "p" => println!("show current position"),
            "g" => println!("{}", game_string_constants::GUIDE),
            "c" => println!("{}", game_string_constants::COMMANDS),
            "l" => println!("{}", game_string_constants::MAP_LEGEND),
            "d" => println!("change difficulty"),
            _ => println!("Command not available!"),
        }

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
