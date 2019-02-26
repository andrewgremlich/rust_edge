#[derive(Debug)]
pub struct Player {
    x_player_position: u8,
    y_player_position: u8,
    lives: u8,
}

// impl Player {}

pub fn init_player(x_player_position: u8, y_player_position: u8, lives: u8) -> Player {
    let player: Player = Player {
        x_player_position: x_player_position,
        y_player_position: y_player_position,
        lives: lives,
    };

    player
}
