/*
See notes on metadatum attributes and structs in ./src/game/game_config.rs
*/
#[derive(Debug)]
struct Map {
    x_length: u8,
    y_length: u8,
    x_goal: u8,
    y_goal: u8,
    number_of_dangers: u8,
    show_map: bool,
}

// impl Map {}

/*
A public function to initialize the map from the struct given in this file.
*/
pub fn init_map(
    x_length: u8,
    y_length: u8,
    x_goal: u8,
    y_goal: u8,
    num_of_dangers: u8,
    show_map: bool,
) {
    /* Init struct */
    let map: Map = Map {
        x_length: x_length,
        y_length: y_length,
        x_goal: x_goal,
        y_goal: y_goal,
        number_of_dangers: num_of_dangers,
        show_map: show_map,
    };

    println!("{:?}", map);
}
