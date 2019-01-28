mod structs;
use crate::structs::Rectangle;

#[allow(unused_variables, dead_code)]
fn main() {
    println!("Hello world");
}

pub fn add_two(num: i32) -> i32 {
    num + 2
}

pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
    //String::from("Hello!")    // This would fail the test
}

struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {

        // Splitting it up will
        if value < 1 {
            panic!("Guess value must be greater than or equal to 1, got {}.", value);
        } else if value > 100 {
            panic!("Guess value must be less than or equal to 100, got {}.", value);
        }

        Guess {
            value
        }
    }
}

#[cfg(test)]
mod tests {
    // Is there another way to do this?
    use super::*;

    // NOTE: Since this is returning a Result, you cannot have the #[should_panic] with this function
    #[test]
    fn result_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }

    #[test]
    #[should_panic(expected = "Guess value must be less than or equal to 100")]
    fn greater_than_100() {
        Guess::new(200);
    }

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was `{}`", result
        );
    }

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
    #[test]
    fn another_one() {
        assert_eq!(4 + 4, 8);
    }
//    #[test]
//    fn fails() {
//        panic!("This test will fail");
//    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle { width: 7, height: 8 };
        let smaller = Rectangle { width: 1, height: 5 };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle { width: 7, height: 8 };
        let smaller = Rectangle { width: 1, height: 5 };

        assert!(!smaller.can_hold(&larger));
    }
}


