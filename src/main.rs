mod guessing_game;

// crate refers to current crate.
// without it, you're looking for an external crate
use crate::guessing_game::{
    guessing_game,
    Guess
};

#[allow(unused_variables, dead_code)]
fn main() {

    // This won't work
    // let guess = guessing_game::Guess {value: 200};
    let guess = Guess::new(2);
    guessing_game();

}


