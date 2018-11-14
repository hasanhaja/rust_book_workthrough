mod enums;
mod mod_system;

use enums::Message;
use enums::action;
use mod_system::*;

#[allow(unused_variables, dead_code)]
fn main() {
    let quit = Message::Quit;
    let write = Message::Write(String::from("Hello world"));
    let color = Message::ChangeColor(2,3,4);

    // NOTE: Rust uses snakecase for function names and variable names
    let move_pt = Message::Move {
        x: 23,
        y: 13,
    };

    let ignore = Message::Ignore;

    let some_u8_value = Some(3u8);

    if let Some(3) = some_u8_value {
        println!("threeeeeeeee");
    } else {
        // if let could also have an else block
        // However, I don't why when the match block is more concise
    }

    // This is identical to this match block
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
    }
    // but it is much less verbose. It's basically some syntax sugar
    // However, you lose the exhaustive pattern matching

    // SIDENOTE: I think the variables below are identical
    let a = "hello".to_string();
    // I think the method above just invokes from
    let b = String::from("hello");
    // Therefore, String::from will be faster than to_string

    action(&quit);
    action(&write);
    action(&color);
    action(&move_pt);
    action(&ignore);

    quit.call();
    write.call();
    color.call();
    move_pt.call();
    ignore.call();

    mod_system::print_greeting();
    mod_system::sub_mod::print_another_greeting("Hello");

}


