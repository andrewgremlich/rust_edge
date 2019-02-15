use serde_json::Value;
use std::env;

//Maybe use TUI_RS?  https://github.com/fdehau/tui-rs
// Another one... https://github.com/gyscos/Cursive
// and another one...https://github.com/redox-os/termion
//  They are all pretty even.

fn main() {
    let args: Vec<String> = env::args().collect();
    let difficulty = &args[1];

    let game_config_string = include_str!("config.json");
    let game_config: Value = serde_json::from_str(game_config_string).unwrap();

    println!("{}", game_config[difficulty]);
}
