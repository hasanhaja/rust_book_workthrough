use std::fs::File;
use std::io::ErrorKind;

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

}