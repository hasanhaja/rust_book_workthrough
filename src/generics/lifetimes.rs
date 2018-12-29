use std::fmt::Display;

#[allow(unused_variables, dead_code)]
fn lifetime_example() {

    let r;

    {
        let x = 5;
        r = &x;
    }

    // This is a dangling pointer and will not work
    // println!("r: {}", r);

}

#[derive(Debug)]
pub struct ImportantExcerpt<'a> {
    part: &'a str,
}

// Kinda figured it out on my own after some trial and error and it also makes sense
impl<'a> ImportantExcerpt<'a> {
    pub fn new(part: &str) -> ImportantExcerpt {
        ImportantExcerpt {
            part
        }
    }
    pub fn level(&self) -> i32 { 3 }

    pub fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

pub fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {

    if x.len() > y.len() {
        x
    } else {
        y
    }

}

pub fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
    where T: Display {

    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }

}