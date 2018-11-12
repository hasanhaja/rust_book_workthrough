mod slice;

use slice::first_word;

#[allow(unused_variables, dead_code)]
fn main() {
    let greeting = String::from("Hello world");
    // Because first_word() works on slices of Strings
    let word = first_word(&greeting[..]);

    // And obviously it would also work on &str
    let literal_greeting = "Hello world";
    let word = first_word(&literal_greeting[..]);

    // Because string literals are slices of Strings
    // NOTE: You don't need the slice syntax for this!
    let word = first_word(literal_greeting);
}


