#[derive(Debug)]
pub struct Player {
    x_player_position: u8,
    y_player_position: u8,
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
}
