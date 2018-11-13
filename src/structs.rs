// The use of lifetimes is required to have fields that
// are references to others (e.g. &str)
// Lifetimes will be covered in a later chapter
pub struct Person {
    pub name: String,
    pub age: u64,
    pub email: String,
}

#[derive(Debug)]
pub struct Triangle {
    pub base: u64,
    pub height: u64,
}

impl Triangle {
    // Method syntax
    pub fn area(&self) -> f64 {
        // Oh, and this is how casting works in Rust
        // This is exactly how the syntax works in Kotlin
        (self.base as f64)*(self.height as f64)*0.5
    }
}

#[derive(Debug)]
pub struct Rectangle {
    pub width: u64,
    pub height: u64,
}

impl Rectangle {

    // This is known as associated function and not a method
    // Similar to from() in String::from()
    // This is used to create constructors
    pub fn square(size: u64) -> Rectangle {
        Rectangle { width: size, height: size }
    }
    pub fn area(&self) -> u64 {
        self.width*self.height
    }

    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}