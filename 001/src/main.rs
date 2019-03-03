fn main() {
    // cargo run
    println!("Setup & Primitives");


    let x = 5; // Immutable Variable
    let mut y = 6; // Mutable Variable


    // Integers:
    // 2 major classifications -> Signed & Unsigned
    // i8, u8, i16, u16, i32, u32, i64, u64, isize, usize
    // the number defines the amount of memory that will be used [in bits]
    // isize and usize : the actual memory size of these will be based on the computer that is running it [32bit/64bit etc. depending on architecture of the machine]


    // Float:
    // f32, f64
    // 32bit for sigle precision and 64bit for double precision


    // Boolean:
    // Denoted by 'bool' takes values as either 'true' or 'false'
    let b = true;

    // Character:
    // ASCII Values
    // Denoted by enclosing in single quotes ''
    let c1 = 'a'; /* or */ let c2: char = 'z';

    // Tuples:
    // These are essentially collection of data and they dont need to be of the same type
    let t: (i32, f64, char) = (42, 6.12, 's');
    // Destructuring,
    let (x, _, z) = t;
    // The above statement will match and accordingly assign x & z while ignoring the middle value
    // We can acess tuples as: t.0 or t.1 or t.2


    // Arrays:
    // These are lists of only one type. They are indexed from 0 in Rust
    let arr = [1,2,3,4,5,6,7,8];
    let a1 = arr[0]; // gives a1, 1


    // Vectors another interesting type...


    // Basic Maths:
    let a = 1 + 20; // Addition
    let s = 30 - 28; // Subtraction
    let m = 5 * 10; // Multiplication
    let d = 4 / 6; // Division
    let r = 49 % 2; // Remainder
}
