// OPTIONS:

// Rust Doesnt have the concept of 'null' in it and there is a good reason for that
// "null" as 'they' say is a billion dollar error :P
// Rust instead uses what are called options
// Now 'option' is a basic enum tyoe inside of the standard library
// It would look something like this,

// enum Option<T> {
//     // our enum Option takes in what is called the generic type
//     Some(T),
//     None,
//     // These are the only 2 fields inside our emun

//     // If we pass something to this enum, it will decide whether or not that thing has something or nothing in it...
// }

fn division(x: f64, y: f64 ) -> Option<f64> {
    if y == 0.0 {
        None
    } else {
        Some(x / y)
    }
}

fn main() {
    let res = division(5.0, 7.0);
    match res {
        Some(x) => println!("{:.4}", x), // Debug flag :.'n' to limit the amount of decimal values to 4, basically precision speicifier
        None => println!("Cannot Divide by Zero"),
    }
}
