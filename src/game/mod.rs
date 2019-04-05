/** See mod note in src/main.rs */
mod command;
mod game_configurator;
mod game_string_constants;
mod map;
mod player;

/* Bring into scope of this file. */
use command::Command;
use game_configurator::{get_game_config, GameConfig};
use game_string_constants::{COMMANDS, GUIDE, MAP_LEGEND};
use map::Map;
use player::Player;
use std::io::{stdin, Result};

/* Immitate a CLI program */
pub fn looper() -> Result<()> {
    /* Fetch the game config from game ENV. */
    #[allow(non_snake_case)]
    let GameConfig {
        xPlayerStart,
        yPlayerStart,
        lives,
        xLength,
        yLength,
        xGoal,
        yGoal,
        numDangers,
        showMap,
    } = get_game_config();

    /*
    The game_config, for some reason can't be easily passed through to the map
    function probably because of scopes and ownership of the IndivdiualConfig struct.
    So I'm passing the properties that I need from the game_config struct.
    */
    let mut player_one = Player::new(xPlayerStart, yPlayerStart, lives, 0);

    let mut map_one = Map::new(
        xPlayerStart,
        yPlayerStart,
        xLength,
        yLength,
        xGoal,
        yGoal,
        numDangers,
        showMap,
    );

    println!("{}", GUIDE);
    println!("{}", COMMANDS);

    /*
    String ownership in this function scope.  It'll be changed by the STDIN.
    String datatype can be changed whenever.
    */
    let mut user_input = String::new();

    loop {
        /*
        Locks this handle and reads a line of user_input into the specified buffer.
        See the above note about the `unwrap` function.
        */
        stdin().read_line(&mut user_input).unwrap();

        let Command {
            move_command,
            x_command,
            y_command,
        } = Command::new(&user_input);

        match move_command {
            'm' => {
                map_one.print_map();
                player_one.is_nearby_danger(&map_one.dangers);
            }
            'a' => {
                let is_adjacent_move: bool = player_one.is_adjacent_move((x_command, y_command));
                if is_adjacent_move {
                    player_one.change_map_position((x_command, y_command));
                    map_one.update_player_position((
                        player_one.x_player_position,
                        player_one.y_player_position,
                    ));
                    player_one.is_dead(&map_one.dangers);
                    map_one.update_player_position((
                        player_one.x_player_position,
                        player_one.y_player_position,
                    ));
                    map_one.print_map();
                }
                player_one.is_nearby_danger(&map_one.dangers);
            }
            's' => {
                map_one.mark_suspected_danger((x_command, y_command));
                map_one.print_map();
            }
            'r' => player_one.is_nearby_danger(&map_one.dangers),
            'p' => player_one.print_current_position(),
            'g' => println!("{}", GUIDE),
            'c' => println!("{}", COMMANDS),
            'l' => println!("{}", MAP_LEGEND),
            'E' => println!("Error parsing first character of command."),
            _ => println!("Command not available!"),
        }

        /* Overwrite previous STDIN */
        user_input.clear();

        if player_one.player_won_game(&(map_one.x_goal, map_one.y_goal)) {
            println!("You indeed found Attila the Hun! You win!");
            break;
        }

        if player_one.lives < 0 {
            println!("You lose!");
            break;
        }

        /* If "exit" is typed then stop the program. */
        if user_input == "exit\n" {
            break;
        }
    }

    /* Return a tuple for the `io::Result` of this function. */
    Ok(())
}
