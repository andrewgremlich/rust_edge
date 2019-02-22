use std::io;

/**
 * Rust looks for ./map.rs or map/mod.rs
 */
mod map;

/* Immitate a CLI program */
pub fn looper() -> io::Result<()> {
    /* String ownership in this function scope.  It'll be changed by the STDIN */
    let mut user_input = String::new();

    map::foo();

    loop {
        /*
        Locks this handle and reads a line of user_input into the specified buffer.
        See the above note about the `unwrap` function.
        */
        io::stdin().read_line(&mut user_input).unwrap();
        println!("You typed: {}", user_input.trim());

        /* Overwrite previous STDIN */
        user_input.clear();

        /* If "exit" is typed then stop the program. */
        if user_input == "exit\n" {
            break;
        }
    }

    /* Return a tuple for the `io::Result` of this function. */
    Ok(())
}
