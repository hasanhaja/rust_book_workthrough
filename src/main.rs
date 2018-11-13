mod enums;

use enums::Message;
use enums::action;

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
}


