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

    pub fn change_map_position(&mut self, command: (u8, u8)) {
        self.x_player_position = command.0;
        self.y_player_position = command.1;
    }
}
