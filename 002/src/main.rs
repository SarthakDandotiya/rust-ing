// Strings, Tuples, Arrays, Slices and Pretty Printing
use std::mem; //Bringing in a library
fn main() {
    // Deeper dive

    // We can put a tuple inside of a tuple
    let t = (1, 'a', false);
    let f = (2, t); // Equivalent to: let f = (2, (1, 'a', false);
    println!("{} {} {}", t.0, t.1, t.2);
    // println!("{}", f.1); is going to give an error...we could say that the compiler doesnt know how to do so...
    // However we can do this but it will ouput a little different
    println!("{:?}", f);
    // The :? is called the 'debug flag' and since tuples have a debug trait, we can do this
    println!("{:#?}", f);// Another way to print the tuple but in a more pretty way, essentially we could thing of this as a pretty debug flag
    // However if there are too many values in the tuple, the compiler will not be able to handle it hence throwing an error
    // So we'd have to implement the printing manually


    let xs: [i32; 5] = [4,5,6,7,8]; // We can give arrays a type signature...here it basically says that the array is filled with i23 integers and its length is 5
    println!("{}", xs[0]); // We can acess the values inside of an array
    println!("{}", xs.len()); // We can also acess the arrays length
    println!("{}, Length = {} Memory = {}", xs[0], xs.len(), mem::size_of_val(&xs));
    // The last part above tells us how big the array is in the memory (in bytes)
    let ys = &xs[2..4]; // Slicing from 2nd to 3rd index
    // & - Stands for reference

    println!("{} {}", ys[0], ys[1]);
    // We can use the debug flags to print out arrays and slices
    println!("{:?} {:?}", ys, xs); // Slice then the whole Array


    let s = "String"; //You would think that this is actually a string but in reality this what is a Slice of a string
    let ss = String::from("String!");
    // In the above case s is &str where as ss is std::string::String -> The string type
    // So, in Rust there is actually a difference when creating a String using double quotes and actually using a function to create a string, because strings are not technically literal types in Rust.
    // Infact Strings are more like arrays or tuples in the sense that they are compound types [made up of multiple different characters]
    // We can convert s into a string as,
    let sx = "String".to_string();
    
    let s_slice = &ss[0..4]; // Slice of a String
    println!("{}", s_slice);

    let h = String::from("Hello, ");
    let w = String::from("World!");
    let conc = h + &w; // Concatenation of 2 Strings
    // Here we use the reference for w
    println!("{}", conc);


    let t = (); // This is a unit value or an empty tuple
    // our main() return empty tuples...functions that don't return anything actually return an empty tuple
}
