fn main() {
    // Ownership & Borrowing

    // Ownership in Rust:

    // Each Variable has a value and the variable is itself called an owner
    let x = 1; // We can say that x owns 1 and here since 1 is a literal, its stored on the stack and not the heap...

    let y = x; // Each piece of data can only have one owner at a time, when the owner goes out of scope the value will be dropped
    // Creating a new scope referenced by the curly brackets
    {
        let a = 10;
    }
    // x + a;
    // If we try to do something with 'a' then that will lead to a problem as 'a' isnt in the scope as 'a' only exists inside of those curly braces
    // We can make multiple scopes using the curly braces

    // Ownership becomes more important when dealing with complex types that are stored on the heap
    let s = String::from("Sarthak"); // Here s owns the strind
    // let y = s; on binding s to y
    // println!("{}", s); this will lead to an error as we have moved the reference of the string from s to y and then all the refence to s actually disappears from the scope
    // only one reference can own the piece of data at a time and at this moment thats exactly what is happening

    // Now to fix the issue above, we use Borrowing
    let y = &s; // This means y = a reference of s
    // Now we can print out s even after declaring y
    println!("{}", s);


}
