use std::env;

const USER_DIFFICULTY: &str = "DIFF";

/*
There are structs in this file.  The structs are essentially
schemas for objects, or prototype constructors in Javascript.
*/

/*
These hashes are metadatum attributes.  They do specific changes to
whatever they are attached to.

IndividualConfig struct is used per game configuration difficulty.
*/
#[allow(non_snake_case)]
pub struct GameConfig {
    pub yLength: u8,
    pub xLength: u8,
    pub yPlayerStart: u8,
    pub xPlayerStart: u8,
    pub yGoal: u8,
    pub xGoal: u8,
    pub numDangers: u8,
    pub lives: i8,
    pub showMap: bool,
}

#[allow(non_snake_case)]
impl GameConfig {
    fn new(
        yLength: u8,
        xLength: u8,
        yPlayerStart: u8,
        xPlayerStart: u8,
        yGoal: u8,
        xGoal: u8,
        numDangers: u8,
        lives: i8,
        showMap: bool,
    ) -> GameConfig {
        GameConfig {
            yLength,
            xLength,
            yPlayerStart,
            xPlayerStart,
            yGoal,
            xGoal,
            numDangers,
            lives,
            showMap,
        }
    }
}

pub fn get_game_config() -> GameConfig {
    let selected_difficulty: &str = &env::var(USER_DIFFICULTY).unwrap();

    match selected_difficulty {
        "easy" => GameConfig::new(10, 10, 0, 0, 8, 8, 12, 5, true),
        "medium" => GameConfig::new(12, 12, 1, 1, 10, 10, 30, 3, true),
        "hard" => GameConfig::new(14, 14, 2, 2, 12, 12, 60, 1, true),
        "ludicrous" => GameConfig::new(14, 14, 2, 2, 12, 12, 60, 0, false),
        _ => GameConfig::new(0, 0, 0, 0, 0, 0, 0, 0, false),
    }
}
