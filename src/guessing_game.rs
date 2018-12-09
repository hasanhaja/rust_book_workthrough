use std::{
    cmp::Ordering,
    io,
};
use rand::Rng;

pub struct Guess {
    value: i32, // this private, so it cannot be accessed or set outside this module
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, but got {}", value);
        }

        Guess {
            value
        }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}

pub fn guessing_game() {
    println!("Guess the number!\nThe secret number is between 1 and 100!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
                .expect("Failed to read line");

        let guess: Guess = match guess.trim().parse() {
            Ok(num) => Guess::new(num),
            Err(_) => {
                println!("Please enter a valid number!");
                continue;
            }
        };

        // This will be too tedious, so we can introduce a struct and handle this logic there
//        if guess < 1 || guess > 100 {
//            println!("This secret number will be in between 1 and 100");
//            continue;
//        }

        println!("You guessed : {}", guess.value());

        match guess.value().cmp(&secret_number ) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
