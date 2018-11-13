mod structs;

use structs::*;
#[allow(unused_variables, dead_code)]
fn main() {
    let hasan = Person {
        name: String::from("Hasan"),
        email: String::from("hasan@haja.com"),
        age: 84,
    };

    println!("{0}'s email is: {1}.\n{0}'s {2} years old", hasan.name, hasan.email, hasan.age);

    let triangle = Triangle {
        base: 8,
        height: 10,
    };

    let rect = Rectangle {
        width: 10,
        height: 12,
    };

    let square = Rectangle::square(5);

    println!("{:?}", square);
    println!("{}", square.area());

    println!("{}", rect.can_hold(&square));

    println!("{:?}", triangle);
    println!("{:#?}", triangle);
    //println!("Area of the triangle is {}.", area(&triangle));
    println!("{}", triangle.area());

}


