/* Bring into scope these crates/modules */
use serde::Deserialize;
use serde_json;
use std::env;

/*
`const`s are variable constants.
`&str` are references to the String wherever it is.
*/
const CONFIG: &str = "GAME_CONFIG";
const USER_DIFFICULTY: &str = "DIFF";

#[derive(Deserialize, Debug)]
struct Gameconfig {
    easy: IndivdiualConfig,
    medium: IndivdiualConfig,
    hard: IndivdiualConfig,
    ludicrous: IndivdiualConfig,
}

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

    let diff_game_config: IndivdiualConfig = match difficulty.trim() {
        "easy" => game_config.easy,
        "medium" => game_config.medium,
        "hard" => game_config.hard,
        "ludicrous" => game_config.ludicrous,
        _ => game_config.easy,
    };

    diff_game_config
}
