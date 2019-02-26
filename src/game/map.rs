/*
See notes on metadatum attributes and structs in ./src/game/game_config.rs
*/
#[derive(Debug)]
pub struct Map {
    x_player_position: u8,
    y_player_position: u8,
    x_length: u8,
    y_length: u8,
    x_goal: u8,
    y_goal: u8,
    number_of_dangers: u8,
    show_map: bool,
    map_marks: Vec<Vec<char>>,
    visited_map_coor: Vec<(u8, u8)>,
}

/* Implementation of Map.  Provide functions and or values only to Map struct. */
impl Map {
    pub fn new(
        x_player_start: u8,
        y_player_start: u8,
        x_length: u8,
        y_length: u8,
        x_goal: u8,
        y_goal: u8,
        num_of_dangers: u8,
        show_map: bool,
    ) -> Map {
        let mut map = Map {
            x_player_position: x_player_start,
            y_player_position: y_player_start,
            x_length: x_length,
            y_length: y_length,
            x_goal: x_goal,
            y_goal: y_goal,
            number_of_dangers: num_of_dangers,
            show_map: show_map,
            map_marks: Vec::new(),
            visited_map_coor: Vec::new(),
        };

        map.visited_map_coor.push((2, 3));
        map.visited_map_coor.push((3, 3));

        Map::generate_map(&mut map);

        map
    }

    fn generate_explored(&mut self) {
        let y_position = self.y_player_position as usize;
        let x_position = self.x_player_position as usize;

        self.map_marks[y_position][x_position] = '&';

        for n in &self.visited_map_coor {
            let x_visited_position = n.0 as usize;
            let y_visited_position = n.1 as usize;
            self.map_marks[y_visited_position][x_visited_position] = '*';
        }
    }

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
        Map::generate_explored(self);
    }
}
