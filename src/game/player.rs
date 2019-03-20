#[derive(Debug)]
pub struct Player {
    pub x_player_position: u8,
    pub y_player_position: u8,
    pub lives: i8,
    nearby_dangers: u8,
}

impl Player {
    pub fn new(
        x_player_position: u8,
        y_player_position: u8,
        lives: i8,
        nearby_dangers: u8,
    ) -> Player {
        Player {
            x_player_position: x_player_position,
            y_player_position: y_player_position,
            lives: lives,
            nearby_dangers: nearby_dangers,
        }
    }

    fn diff_two_coordinates(&self, command: &(u8, u8)) -> (u8, u8) {
        let mut x_move_difference: i8 = self.x_player_position as i8 - command.0 as i8;
        let mut y_move_difference: i8 = self.y_player_position as i8 - command.1 as i8;

        x_move_difference = x_move_difference.abs();
        y_move_difference = y_move_difference.abs();

        (x_move_difference as u8, y_move_difference as u8)
    }

    pub fn player_won_game(&self, map_goal: &(u8, u8)) -> bool {
        let coordinates_difference = self.diff_two_coordinates(map_goal);

        println!("{:?}", coordinates_difference);

        if coordinates_difference == (0, 0) {
            return true;
        }

        return false;
    }

    pub fn print_current_position(&mut self) {
        println!(
            "\n({},  {})",
            self.x_player_position, self.y_player_position
        );
        println!(
            "Your X position is {} and your Y position is {}\n",
            self.x_player_position, self.y_player_position
        );
    }

    pub fn change_map_position(&mut self, command: (u8, u8)) {
        self.x_player_position = command.0;
        self.y_player_position = command.1;
    }

    pub fn is_dead(&mut self, map_dangers: &Vec<(u8, u8)>) -> bool {
        for danger in map_dangers {
            let coordinates_difference = self.diff_two_coordinates(danger);

            if coordinates_difference.0 == 0 && coordinates_difference.1 == 0 {
                println!("You dead bro!");
                self.lives = self.lives - 1;
                self.x_player_position = 0;
                self.y_player_position = 0;
                return false;
            }
        }

        return true;
    }

    pub fn is_adjacent_move(&self, command: (u8, u8)) -> bool {
        let coordinates_difference = self.diff_two_coordinates(&command);

        if coordinates_difference.0 > 1 || coordinates_difference.1 > 1 {
            println!("Can not move farther than one square!");
            return false;
        } else if coordinates_difference.0 == 1 || coordinates_difference.1 == 1 {
            println!("Moving to another position...");
            return true;
        }

        println!("Something went wrong with the logic here...");
        return false;
    }

    pub fn is_nearby_danger(&self, map_dangers: &Vec<(u8, u8)>) {
        let mut num_nearby_dangers: u8 = 0;

        for danger in map_dangers {
            let coordinates_difference = self.diff_two_coordinates(danger);

            if coordinates_difference.0 <= 1 && coordinates_difference.1 <= 1 {
                num_nearby_dangers = num_nearby_dangers + 1;
            }
        }

        println!("Danger(s) nearby {:?}.", num_nearby_dangers);
    }
}
