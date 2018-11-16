pub enum TableCells {
    Int(i32),
    Float(f64),
    Text(String),
}

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