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

    pub fn change_map_position(&mut self, user_input: &String) {
        let parsed = user_input.split(" ");
        let commands = parsed.collect::<Vec<&str>>();

        if commands.len() != 3 {
            println!("Not enough move arguments!");
            return;
        }

        let x_command: u8 = commands[1].trim().parse().unwrap();
        let y_command: u8 = commands[2].trim().parse().unwrap();

        self.x_player_position = x_command;
        self.y_player_position = y_command;

        println!("{:?}", self);
    }
}
