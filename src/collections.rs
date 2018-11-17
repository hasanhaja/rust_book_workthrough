pub enum TableCells {
    Int(i32),
    Float(f64),
    Text(String),
}

#[allow(unused_variables, dead_code)]
pub fn enum_vectors() {

    let v = vec![
        TableCells::Int(2),
        TableCells::Float(23.3),
        TableCells::Int(2),
        TableCells::Text(String::from("Bob the builder")),
    ];

    for i in &v {
        match i {
            TableCells::Int(value) => println!("{}", value),
            TableCells::Text(value) => println!("{}", value),
            _ => {},
        }
    }

}

#[allow(unused_variables, dead_code)]
pub fn vector_loops() {
    let v: Vec<i32> = Vec::new();
    let v = vec![1,2,3,4];

    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    v.push(9);
    v.push(10);

    let v = vec![1,2,3,4,5,6];
    let fourth: &i32 = &v[2];   // This will panic if out of range
    let v_index = 22;

    // get method can also be used to get element at index
    // Since this returns an Option, it doesnt panic
    match v.get(v_index) {
        Some(_) => println!("Reachable element at index: {}", v_index),
        None => println!("Unreachable element at index: {}", v_index),
    }

    let v = vec![100, 12, 54, 43];

    for i in &v {
        println!("{}", i);
    }

    let mut v = vec![100, 12, 54, 43];

    for i in &mut v {

        // * is derefencing; you are accessing the value the pointer is pointing to
        *i += 50;
        println!("{}", i);
    }
}

#[allow(unused_variables, dead_code)]
pub fn string_ops() {

    let s1 = String::from("Hello");
    let s2 = String::from(", world!");

    //let s3 = s1 + &s2;

    let s1 = String::from("Hello");
    let s2 = String::from(", world!");

    let s3 = s1 + &s2;
    // The way this works is by invoking the add function
    // The actual function is implemented with generics
    // but it looks like this: fn add(self, s: &str) -> String
    // That is a reason s1 ownership is transfered and s2 is borrowed.
    // However, the reason we can pass in a &String instead
    // of a &str is because of deref coersion.
    // ASIDE: That would've looked like &s2[..]

    println!("{}", s3);

    let tic = String::from("Tic");
    let tac = String::from("Tac");
    let toe = String::from("Toe");

    // a prettier, less unwieldly form of this would be
    //let s = tic + "-" + &tac + "-" + &toe;

    // Also this does not take ownership of tic.
    let s = format!("{}-{}-{}", tic, tac, toe);
    // format! works like println! but it returns a String

    println!("{}", s);

    let hello = "Здравствуйте";

    // you need to ensure the range is not in the char boundary
    let s = &hello[0..4];
    println!("{}", s);

    // to iterate over strings you use the char method
    for c in "hasan".chars() {
        println!("{}", c);
    }

    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

}

#[allow(unused_variables, dead_code)]
pub fn hashmap_ops() {

    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = match scores.get(&team_name) {
        Some(value) => println!("{}", value),
        None => (),
    };

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // To create a hashmap from a vector of tuples
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    // The reason you need to specify the type is because collect can
    // collect into various different data structures
    // for the params, we use underscores so rust can infer the types
    let scores: HashMap<_,_> = teams.iter()
                                    .zip(initial_scores.iter())
                                    .collect();


    // For types that implement the Copy trait, the values are copied into the hashmap
    // For types that don't the values are moved, and hashmap becomes the owner.

    let field_name = String::from("Fav food");
    let field_value = String::from("Curry");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);    // because String doesn't impl Copy, their values are moved.

    // println!("{}", field_name);      // therefore, this does not work!

    // keys can have only one value associated with it
    // to overwrite values
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);    // this overwrites the previous statement

    println!("{:?}", scores);

    // to insert values only when the key has no value
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);   // because Blue already had a value, this did nothing
    // .entry() returns an Enum Entry that represents the value

    println!("{:?}", scores);   // Hashmap doesn't maintain order

    let text = "hello potato nice potato";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;       // since count has a type of mutable reference
                           // you deref it to update the value
        // .or_insert returns a  mutable reference to the value of the key
    }

    println!("{:?}", map);

}

// TODO("Do the exercises at the end of chapter 8")
#[allow(unused_variables, dead_code)]
pub fn hashmap_exercise() {
    unimplemented!();
}