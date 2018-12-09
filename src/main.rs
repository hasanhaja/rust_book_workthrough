mod errors;

use std::{
    error::Error,
    fs::File,
};

// Since ? only works for functions returning Result, you can make main return Result
// Box<dyn Error> is a trait object (will be discussed in Ch17).
// For now, read this as "any kind of Error"
#[allow(unused_variables, dead_code)]
fn main() -> Result<(), Box<dyn Error>> {

    errors::recoverable();

    let f = File::open("hello.txt")?;

    Ok(())

}


