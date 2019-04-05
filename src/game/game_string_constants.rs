/*
&str constants to be used where ever needed in mod.rs.
*/

pub const GUIDE: &str = "
/*********************************************/
      Welcome to Rust Edge!
/*********************************************/

You are a daring explorer with a rust edge
sword.  You are exploring a tomb that is 
believed to hold Attila the Hun!

To find your way through the tomb, you must
avoid the dangers along the way.  You will
receive clues on your map when you are next to
danger, but the exact spot of the danger is
unknown!

Be careful on this great adventurous endeavor!
The map is a 2d grid, with (0,0) starting
in the top left corner.
";

pub const COMMANDS: &str = "
Available commands.

***Basic Commands
[m]            Show Map
[a] {x} {y}    Move position (example command 'a 2 2')
[s] {x} {y}    Suspected danger (example command 's 2 2')
[r]            Remind nearby dangers
[p]            Show current position

***Help Commands
[g]    Show guide
[c]    Show commands again
[l]    Show Legend

";

pub const MAP_LEGEND: &str = "
/*****   MAP LEGEND   *****/

* '&'     Player Icon
* 'O'     Get to goal!
* '.'     Unknown location
* '*'     Visited location with no danger.
* 'X'     Visited location with known danger.
* '?'     Location with suspected danger.

* Nearby dangers appear as a number
  * i.e. '1' or '2'.
";
