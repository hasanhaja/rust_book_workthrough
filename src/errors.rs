use std::fs::{
    self,
    File,
};
use std::io::{
    self,
    ErrorKind,
    Read,
};

pub fn recoverable() {

    let path = "hello.txt";

    // The error type is std::io::Error
    let f= File::open(path);

//    let f = match f {
//        Ok(file) => file,
//
//        // This would catch all errors, and not the specific kind
//        Err(error) => {
//            panic!("There was a problem opening the file: {:?}", error)
//        }
//    };


    // This has a lot of match statements.
    // match is a primitive. The result type has many methods that accept closures and are implemented with match statements
    let file_match = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {

            // This is a "NotFound" error kind. This now tries to create a file which too returns a Result type because file creation can fail.
            ErrorKind::NotFound => match File::create(path) {
                Ok(fc) => fc,
                Err(e) => panic!("Tried to create a file but there was a problem: {:?}", e),
            },

            // This handles any other error kinds.
            other_error => panic!("There was a problem opening the file: {:?}", other_error),
        }
    };

    // another way to write this would be written. This would be the way a more experienced Rustacean would do it. Closure will be covered in Ch13
    // TODO("Revisit map_err and unwrap_or_else after chapter 13")
    // The way unwrap works is, if the Result type returns a Ok variant,
    // it'll return the value inside. Otherwise, it'll panic.
    // It is most likely implemented with match statements underneath
    // the except method works in the same way, but you can specify the error msg
    let file_closure = File::open(path).map_err(|error| {
        if error.kind() == ErrorKind::NotFound {

            // My guess is, it is trying to unwrap, but if it runs into an error it panics.
            File::create(path).unwrap_or_else(|error| {
                panic!("Tried to create file but there was a problem: {:?}", error);
            })
        } else {
            panic!("There was a problem opening the file: {:?}", error);
        }
    });

    // NOTE: This is the long-winded way of doing propagating error
    // We chose io::Error because that's what File::open and read_to_string return
    // This is common in rust, that we have the "?" operator
    // ? will only work when in used in functions that return Result
    fn read_username_from_file() -> Result<String, io::Error> {
        let f = File::open("hello.txt");

        let mut f = match f {
            Ok(file) => file,
            Err(e) => return Err(e),    // early return
        };

        let mut s = String::new();

        // this is the final expression that is either returning Ok(s) or Err(e)
        match f.read_to_string(&mut s) {
            Ok(_) => Ok(s),
            Err(e) => Err(e),
        }

    }

    // This is how it can be written
    fn read_othername_from_file() -> Result<String, io::Error> {
        let mut f = File::open("hello.txt")?;   // this works like the match statements on Results
        // if the value is OK, the file would get unwrapped and the program continues
        // otherwise it returns
        let mut s = String::new();
        f.read_to_string(&mut s)?;
        Ok(s)   // return
    }
    // NOTE: There is a difference between how a match statements work and how "?" works.
    // error values taken by ? go through from function implemented by the From trait
    // This converts errors from one type to another.
    // When ? call from, the error type received is converted to the error type defined in the return type of the function (io::Error, in our case)
    // This is useful when a function returns one error type to represent all the ways a function might fail, even if parts fail for different reasons
    // As long as the error type implements from function to define how to convert itself to the returned error type, ? takes care of the conversion automatically.

    // The function above could be further shortened to this
    fn read_name_from_file() -> Result<String, io::Error> {
        let mut s = String::new();

        File::open("hello.txt")?.read_to_string(&mut s)?;

        Ok(s)
    }

    // This makes it even shorter. This is a part of the standard library
    fn read_from_file() -> Result<String, io::Error> {
        fs::read_to_string("hello.txt")
    }

}