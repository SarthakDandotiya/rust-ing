// Basic Casting
fn main() {
    // Rust provides no implicit type conversion or coersion as its called. Between primitive types
    // Instead there is whats called explicit conversion or casting which can be performed with the 'as' keyword 

    let f = 91.4321_f32;
    let i = f as u8;
    let c = i as char;
    // While converting from integer to character, we need to use a u8 this means our integer can only be from 0 to 255

    println!("{} {} {}", f, i, c);
}
