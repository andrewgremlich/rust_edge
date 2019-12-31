/*
There are structs in this file.  The structs are essentially
schemas for objects, or prototype constructors in Javascript.
*/

/*
These hashes are metadatum attributes.  They do specific changes to
whatever they are attached to.

IndividualConfig struct is used per game configuration difficulty.
*/

pub struct GameConfig {
    pub y_length: u8,
    pub x_length: u8,
    pub y_player_start: u8,
    pub x_player_start: u8,
    pub y_goal: u8,
    pub x_goal: u8,
    pub num_dangers: u8,
    pub lives: i8,
    pub show_map: bool,
}


impl GameConfig {
    pub fn new(diff: &str) -> GameConfig {
        match diff {
            "easy" => GameConfig {
                y_length: 10,
                x_length: 10,
                y_player_start: 0,
                x_player_start: 0,
                y_goal: 8,
                x_goal: 8,
                num_dangers: 12,
                lives: 5,
                show_map: true,
            },
            "medium" => GameConfig {
                y_length: 12,
                x_length: 12,
                y_player_start: 1,
                x_player_start: 1,
                y_goal: 10,
                x_goal: 10,
                num_dangers: 30,
                lives: 3,
                show_map: true,
            },
            "hard" => GameConfig {
                y_length: 14,
                x_length: 14,
                y_player_start: 2,
                x_player_start: 2,
                y_goal: 12,
                x_goal: 12,
                num_dangers: 60,
                lives: 1,
                show_map: true,
            },
            "ludicrous" => GameConfig {
                y_length: 14,
                x_length: 14,
                y_player_start: 2,
                x_player_start: 2,
                y_goal: 12,
                x_goal: 12,
                num_dangers: 60,
                lives: 0,
                show_map: false,
            },
            _ => GameConfig {
                y_length: 0,
                x_length: 0,
                y_player_start: 0,
                x_player_start: 0,
                y_goal: 0,
                x_goal: 0,
                num_dangers: 0,
                lives: 0,
                show_map: true,
            },
        }
    }
}
