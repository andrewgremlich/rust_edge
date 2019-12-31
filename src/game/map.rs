use rand::Rng;
/*
See notes on metadatum attributes and structs in ./src/game/GameConfig.rs
*/
#[derive(Debug)]
pub struct Map {
    x_player_position: u8,
    y_player_position: u8,
    x_length: u8,
    y_length: u8,
    pub x_goal: u8,
    pub y_goal: u8,
    number_of_dangers: u8,
    pub dangers: Vec<(u8, u8)>,
    show_map: bool,
    map_marks: Vec<Vec<char>>,
    visited_map_coor: Vec<(u8, u8)>,
    suspected_danger: Vec<(u8, u8)>,
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
            dangers: Vec::new(),
            show_map: show_map,
            map_marks: Vec::new(),
            visited_map_coor: Vec::new(),
            suspected_danger: Vec::new(),
        };

        map.visited_map_coor.push((x_player_start, y_player_start));

        map.generate_dangers();
        map.generate_map();

        map
    }

    pub fn print_map(&self) {
        let _can_do_map = match &self.show_map {
            true => self.output_map(),
            false => println!("You lost the map!"),
        };
    }

    pub fn update_player_position(&mut self, command: (u8, u8)) {
        self.x_player_position = command.0;
        self.y_player_position = command.1;

        if !self.visited_map_coor.contains(&command) {
            self.visited_map_coor.push(command);
        }

        self.generate_map();
    }

    pub fn mark_suspected_danger(&mut self, suspect_coor: (u8, u8)) {
        self.suspected_danger.push(suspect_coor);
        self.generate_map();
    }

    fn output_map(&self) {
        for n in &self.map_marks {
            for m in n {
                print!("{} ", m);
            }
            print!("\n");
        }
    }

    fn generate_map(&mut self) {
        self.map_marks = Vec::new();

        /* the parameter isn't &self or &mut self, because it already is &mut. */
        self.generate_unexplored();
        self.generate_map_marks();
    }

    fn generate_map_marks(&mut self) {
        let y_position = self.y_player_position as usize;
        let x_position = self.x_player_position as usize;
        let y_goal = self.y_goal as usize;
        let x_goal = self.x_goal as usize;

        for n in &self.visited_map_coor {
            let x_visited_position = n.0 as usize;
            let y_visited_position = n.1 as usize;
            self.map_marks[y_visited_position][x_visited_position] = '*';

            for m in &self.dangers {
                if n.0 == m.0 && n.1 == m.1 {
                    self.map_marks[y_visited_position][x_visited_position] = 'X';
                }
            }
        }

        for n in &self.suspected_danger {
            let x_suspect_coor = n.0 as usize;
            let y_suspect_coor = n.1 as usize;

            self.map_marks[y_suspect_coor][x_suspect_coor] = '?';
        }

        self.map_marks[y_position][x_position] = '&';
        self.map_marks[y_goal][x_goal] = 'O';
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

    fn generate_dangers(&mut self) {
        let mut done = false;

        while !done {
            let x_danger_coor: u8 = rand::thread_rng().gen_range(0, self.x_length);
            let y_danger_coor: u8 = rand::thread_rng().gen_range(0, self.y_length);
            let danger_coor = (x_danger_coor, y_danger_coor);

            if self.dangers.contains(&danger_coor)
                || danger_coor == (self.x_player_position, self.y_player_position)
                || danger_coor == (self.x_goal, self.y_goal)
            {
                continue;
            }

            self.dangers.push(danger_coor);

            if self.dangers.len() as u8 == self.number_of_dangers {
                done = true;
            }
        }
    }
}
