mod generics;

use crate::generics::{
    largest,
    largest_2,
    Point,
    Tweet,
    Summary,
    notify,
};

#[allow(unused_variables, dead_code)]
fn main() {

    let num_list = vec![21, 54, 12, 11, 100, 32];
    let result = largest(&num_list);
    println!("The largest number is {}", result);

    let result = largest_2(&num_list);
    println!("2: The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("The largest number is {}", result);

    let result = largest_2(&char_list);
    println!("2: The largest number is {}", result);


    let p = Point::new(2, 3);
    println!("{}", p.x());

    let tweet = Tweet {
        username: String::from("rust_lang"),
        content: String::from("still learning rust and trying to get productive with it"),
        retweet: false,
        reply: false,
    };

    //notify(&tweet);    // This won't work here, because tweet gets moved here
    println!("1 new tweet: {}", tweet.summarize());
    notify(tweet);


}


