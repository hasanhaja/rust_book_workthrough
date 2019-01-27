mod structs;
use crate::structs::Rectangle;

#[allow(unused_variables, dead_code)]
fn main() {
    println!("Hello world");
}

pub fn add_two(num: i32) -> i32 {
    num + 2
}

#[cfg(test)]
mod tests {
    // Is there another way to do this?
    use super::*;

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


