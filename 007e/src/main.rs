// Result Emun

// Earlier we have discussed the option enum now we'll see an enum called result
// This Result enum is basically used in error chaecking in Rust, we can also use the Option enum to error check as well...

// The difference between Option and Result is that Result will allow to see why something has failed.

// enum Result<T, E> {
//     // <T, E> these are generic types, what it really means is that type inside of Ok() can be different from type inside of Err()
//     Ok(T),
//     Err(E),
// }

use std::fs::File;

fn main() {
    let f = File::open("test.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => {
            panic!("There was an problem while opening the file: {:?}", error)
            // What a panic!() does is that it will kick us out of the program
        },
        // If we do not have the test.txt in the root folder we see this output:
        // thread 'main' panicked at 'There was an problem while opening the file: Os { code: 2, kind: NotFound, message: "No such file or directory" }', src/main.rs:22:13
    };
}
