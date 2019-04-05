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
    pub fn new(diff: &str) -> GameConfig {
        match diff {
            "easy" => GameConfig {
                yLength: 10,
                xLength: 10,
                yPlayerStart: 0,
                xPlayerStart: 0,
                yGoal: 8,
                xGoal: 8,
                numDangers: 12,
                lives: 5,
                showMap: true,
            },
            "medium" => GameConfig {
                yLength: 12,
                xLength: 12,
                yPlayerStart: 1,
                xPlayerStart: 1,
                yGoal: 10,
                xGoal: 10,
                numDangers: 30,
                lives: 3,
                showMap: true,
            },
            "hard" => GameConfig {
                yLength: 14,
                xLength: 14,
                yPlayerStart: 2,
                xPlayerStart: 2,
                yGoal: 12,
                xGoal: 12,
                numDangers: 60,
                lives: 1,
                showMap: true,
            },
            "ludicrous" => GameConfig {
                yLength: 14,
                xLength: 14,
                yPlayerStart: 2,
                xPlayerStart: 2,
                yGoal: 12,
                xGoal: 12,
                numDangers: 60,
                lives: 0,
                showMap: false,
            },
            _ => GameConfig {
                yLength: 0,
                xLength: 0,
                yPlayerStart: 0,
                xPlayerStart: 0,
                yGoal: 0,
                xGoal: 0,
                numDangers: 0,
                lives: 0,
                showMap: true,
            },
        }
    }
}
