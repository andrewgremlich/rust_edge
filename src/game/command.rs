#[derive(Debug)]
pub struct Command {
    pub move_command: char,
    pub x_command: u8,
    pub y_command: u8,
}

impl Command {
    pub fn new(user_input: &String) -> Command {
        let commands_parsed: Vec<&str> = user_input.split_whitespace().collect();

        let letter_command: char = commands_parsed[0].trim().parse::<char>().unwrap();

        let mut x_command: u8 = 0;
        let mut y_command: u8 = 0;

        if commands_parsed.len() > 1 {
            x_command = commands_parsed[1].trim().parse::<u8>().unwrap();
            y_command = commands_parsed[2].trim().parse::<u8>().unwrap();
        }

        Command {
            move_command: letter_command,
            x_command: x_command,
            y_command: y_command,
        }
    }
}
