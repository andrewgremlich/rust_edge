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
    map_marks: Vec<Vec<char>>,
}

/* Implementation of Map.  Provide functions and or values only to Map struct. */
impl Map {
    /* the parameter is &mut self, because this method doesn't know. */
    fn generate_unexplored(&mut self) {
        for _n in 0..self.y_length {
            let mut row = Vec::new();

            for _m in 0..self.x_length {
                row.push('.');
            }

            self.map_marks.push(row);
        }
    }

    fn generate_map(&mut self) {
        /* the parameter isn't &self or &mut self, because it already is &mut. */
        Map::generate_unexplored(self);
    }
}

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
    let mut map: Map = Map {
        x_length: x_length,
        y_length: y_length,
        x_goal: x_goal,
        y_goal: y_goal,
        number_of_dangers: num_of_dangers,
        show_map: show_map,
        map_marks: Vec::new(),
    };

    /*
    map needs to be passed to self in the impl method as a mutable reference
    so that it only modifies the original map.
    */
    Map::generate_map(&mut map);

    println!("{:?}", &map);
}
