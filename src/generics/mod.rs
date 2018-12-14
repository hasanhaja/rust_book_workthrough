pub mod lifetimes;

use std::fmt::Display;
use std::fmt::Debug;

// Takes a slice
// Copy trait ensures the size can be known and can be allocated on the stack
// You can use Clone if you want to remove restrictions (use largest for strings), but it'll use more heap allocations.
// Also, if we return a reference to the slice, we can remove the Copy or Clone and avoid heap allocation
pub fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {

    // The ide lints an error, but this is fine now.
    let mut largest = list[0];

    // TODO("Understand what &item means here and how that's different to item")
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest

}

pub fn largest_2<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

pub struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    pub fn new(x: T, y: T) -> Point<T> {
        Point { x, y }
    }

    pub fn x(&self) -> &T {
        &self.x
    }

}

// These methods will only become available when Point is of type f32
impl Point<f32> {
    pub fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// the param could also be &impl Summary to pass a reference
// You can also return "impl Summary"
// This means "return something that implements Summary trait but I'm not telling you what type it will be
// However, if the method tries to return either NewsArticle or Tweet depending on a conditional, this won't work. This can only be a NewsArticle or Tweet. To do this, you'll need trait objects which will be covered in ch 17.
// The form below is syntax sugar for notify<T: Summary>(item: T) {...}
pub fn notify(item: impl Summary) {
    println!("Breaking new! {}", item.summarize());
}

// specifying multiple trait with "+" and where for clarity
fn notify_mult1(item: impl Summary + Display) { // Placeholder
}

fn notify_mult2<T: Summary + Display>(item: T) {
    // Placeholder
}

fn some_fn1<T: Display + Clone, U: Clone + Debug>(t: T, u: U) -> i32 {
    // Placeholder
    0
}

fn some_fn2<T, U>(t: T, u: U) -> i32
    where T: Display + Clone,
          U: Clone + Debug
{
    // Placeholder
    0
}


pub trait Summary {
    // With no default implementation
    //fn summarize(&self) -> String;
    // default implementation
//    fn summarize(&self) -> String {
//        String::from("(Read more...)")
//    }

    fn summarize_author(&self) -> String;
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }

}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// If the trait is public, then do we not have to declare the method in the impl block as public?
impl Summary for NewsArticle {
//    fn summarize(&self) -> String {
//        format!("{}, by {} ({})", self.headline, self.author, self.location)
//    }

    fn summarize_author(&self) -> String {
        format!("{}", self.author)
    }


}
// leaving an empty {} ensures we are using the default implementation for NewsArticle

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {

    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }

//    fn summarize(&self) -> String {
//        format!("{}: {}", self.username, self.content)
//    }
}

// Just placeholder to demonstrate the concept
trait ToStringHasan {
    // --implementation--
}

// This is called a blanket implementation
// This means implement ToStringHasan for any type that implements Display
// This is how ToString trait in the standard library is implemented/
impl<T: Display> ToStringHasan for T {
    // --implementation--
}