mod generics;

use crate::generics::lifetimes::{
    longest,
    ImportantExcerpt,
    longest_with_an_announcement,
};

#[allow(unused_variables, dead_code)]
fn main() {

    // This will work as intended
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);

    println!("The longest string is: {}", result);

    // Since the lifetimes of s1 and s2 aren't the same, this would not compile.
//    let s1 = String::from("hasan");
//    let result;
//    {
//        let s2 = "khaleel".to_string();
//        result = longest(s1.as_str(), s2.as_str());
//    }
//
//    println!("{}", result);

    let novel = String::from("I come from Des Moines. Somebody had to.");
    let first_sentence = novel.split('.')
        .next()
        .expect("Could not find a '.'");
    let i = ImportantExcerpt::new(first_sentence);
    println!("{:?}", i);
    let j = i.announce_and_return_part("WE, ARE VENOM!");
    println!("{}", j);

    let result = longest_with_an_announcement(string1.as_str(), string2, String::from("HELLLOOOO!!"));

}


