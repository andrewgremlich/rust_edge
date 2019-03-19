#[derive(Debug)]
pub struct Player {
    pub x_player_position: u8,
    pub y_player_position: u8,
    lives: u8,
}

impl Player {
    pub fn new(x_player_position: u8, y_player_position: u8, lives: u8) -> Player {
        Player {
            x_player_position: x_player_position,
            y_player_position: y_player_position,
            lives: lives,
        }
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

    pub fn is_adjacent_move(&self, command: (u8, u8)) -> bool {
        let mut x_move_difference: i8 = self.x_player_position as i8 - command.0 as i8;
        let mut y_move_difference: i8 = self.y_player_position as i8 - command.1 as i8;

        x_move_difference = x_move_difference.abs();
        y_move_difference = y_move_difference.abs();

        if x_move_difference > 1 || y_move_difference > 1 {
            println!("Can not move farther than one square!");
            return false;
        } else if x_move_difference == 1 || y_move_difference == 1 {
            println!("Moving to another position...");
            return true;
        }

        println!("Something went wrong with the logic here...");
        return false;
    }
}
