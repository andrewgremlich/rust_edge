/* Bring into scope these crates */
use serde::Deserialize;
use serde_json;
use std::env;

/*
`const`s are variable constants.
`&str` are references to the String wherever it is.
&str also is a fixed length and can't be changed.
*/
const CONFIG: &str = "GAME_CONFIG";
const USER_DIFFICULTY: &str = "DIFF";

/*
There are structs in this file.  The structs are essentially
schemas for objects, or prototype constructors in Javascript.
*/

/*
These hashes are metadatum attributes.  They do specific changes to
whatever they are attached to.

This `Gameconfig` struct is used for the deserializing of the
game configuration object.
*/
#[derive(Deserialize, Debug)]
struct Gameconfig {
    easy: IndivdiualConfig,
    medium: IndivdiualConfig,
    hard: IndivdiualConfig,
    ludicrous: IndivdiualConfig,
}

/*
These hashes are metadatum attributes.  They do specific changes to
whatever they are attached to.

IndividualConfig struct is used per game configuration difficulty.
*/
#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct IndivdiualConfig {
    pub yLength: u8,
    pub xLength: u8,
    pub yPlayerStart: u8,
    pub xPlayerStart: u8,
    pub yGoal: u8,
    pub xGoal: u8,
    pub numDangers: u8,
    pub lives: u8,
    pub showMap: bool,
}

/*
The small arrow shows what datatype is to be returned to the callee.
*/
pub fn fetch_config() -> IndivdiualConfig {
    /*
    Fetch an environment variable and deserialize the configuration object.

    `env.var(CONFIG) returns a Result<T, E> type.

    NOTE*** To handle Result types, the `unwrap` function is used.  One could
    have used another way like `match` or `unwrap_or_else`

    https://doc.rust-lang.org/std/result/enum.Result.html
    */
    let retrieved_config = env::var(CONFIG).unwrap();
    let difficulty = env::var(USER_DIFFICULTY).unwrap();
    let game_config: Gameconfig = serde_json::from_str(&retrieved_config).unwrap();

    /*
    Whatever the selected difficulty is, `match` the difficulty to the correlated
    game_config object.

    `.trim()` helps the `difficulty` String become a &str, because any quoted set
    of characters is a &str or string slice.

    Another way for the `&str` to work is to have the match statment be in a different
    function and pass `difficulty` by reference.
     */
    let diff_game_config: IndivdiualConfig = match difficulty.trim() {
        "easy" => game_config.easy,
        "medium" => game_config.medium,
        "hard" => game_config.hard,
        "ludicrous" => game_config.ludicrous,
        _ => game_config.easy,
    };

    /*
    leaving out the semi colon allows diff_game_config to be returned to the callee.
    */
    diff_game_config
}
