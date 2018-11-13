pub enum Message {
    // This is an enum variant with no associated data
    Quit,

    // This is an variant with an anonymous struct
    Move {x: i32, y: i32},

    // Anonymous Tuple struct? Includes a single string data associated
    Write(String),

    // Tuple struct. Includes 3 pieces of data associated with this
    ChangeColor(i32, i32, i32),

    // Only to be ignored
    Ignore,
}

// You can also implement methods for enums
impl Message {
    pub fn call(&self) {
        match self {
            Message::Quit => println!("Quiting"),
            Message::Move { x: a, y: b } => println!("Moving x: {} y: {}", a, b),
            Message::Write(s) => println!("{}", s),
            Message::ChangeColor(r,g,b) => println!("Changing color to rgb: {}{}{}", r, g, b),
            _ => println!("These variants don't have an implementation yet"),
        }
    }
}

pub fn action(m: &Message) {
    // I think this is exhaustive pattern matching and destructuring
    match m {
        Message::Quit => println!("Quiting"),
        Message::Move { x: a, y: b } => println!("Moving x: {} y: {}", a, b),
        Message::Write(s) => println!("{}", s),
        Message::ChangeColor(r,g,b) => println!("Changing color to rgb: {}{}{}", r, g, b),

        // handles all the else cases. I think the equivalent Kotlin would be the else in the when statement
        _ => println!("These variants don't have an implementation yet"),
    }
}