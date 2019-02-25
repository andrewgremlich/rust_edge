pub fn foo(command: &str) {
    match command {
        "m" => println!("you want the map."),
        "h" => println!("you want to hike the mountain."),
        "s" => println!("mark suspected danger."),
        "r" => println!("remind of nearby danger"),
        "p" => println!("show current position"),
        "g" => println!("you want the guide to show."),
        "c" => println!("Show available commands"),
        "l" => println!("Show map legend."),
        "d" => println!("change difficulty"),
        _ => println!("Command not available!")
    }
}
