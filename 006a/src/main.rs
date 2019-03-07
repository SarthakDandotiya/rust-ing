// Enums & Options

// ENUMS:
// Emunerations are the perfect thing to discuss now that we have looked at pattern matching
// Enumerations are the custom way to maketypes in Rust sort of like struct but they us to make multiple different variations on a 'type'
// Unlike enumeration in a language like C# enumerations in Rust are more like algebraic data types from F# or union types from Elm


#![allow(dead_code)] // Annotation to stop showing minor issues that dont really hinder the execution


#[derive(Debug)]
enum Direction {
    // Can be...
    // Up(u32), // Tuples
    // Down {x: u32, y: f64}, // Struct
    // Left,// Unit Types
    // Right,

    // // But something that makes a lot of sense is...
    // Up (i32, i32), // (0,1)
    // Down (i32, i32), // (0,-1)
    // Left (i32, i32), // (-1,0)
    // Right (i32, i32), // (0,1)

    // Something still easier ad that makes more sense is to have a separate struct which will be used here
    Up (Point),
    Down (Point),
    Left (Point),
    Right (Point),
}

#[derive(Debug)]
enum Keys {
    UpKey (String),
    DownKey (String),
    LeftKey (String),
    RightKey (String),
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

// Just like structs we can use implementation blocks to tie methods to our enum
impl Direction {
    fn match_direction(&self) -> Keys {
        match *self {
            Direction::Up(_) => Keys::UpKey(String::from("Pressed W")),
            Direction::Down(_) => Keys::DownKey(String::from("Pressed S")),
            Direction::Left(_) => Keys::LeftKey(String::from("Pressed A")),
            Direction::Right(_) => Keys::RightKey(String::from("Pressed D")),
        }
    }
}

impl Keys {
    // This will basically allow us to de-strust the Keys enum type and get the string from within it
    fn destruct(&self) -> &String {
        match *self {
            // the * is to dereference our 'self'
            Keys::UpKey(ref s) => s,
            Keys::DownKey(ref s) => s,
            Keys::LeftKey(ref s) => s,
            Keys::RightKey(ref s) => s,
        }
    }
}

enum Shape {
    Rectangle {width: u32, height: u32},
    Square(u32),
    Circle(f64),
}

impl Shape {
    // This method is allowing for polymorphism...we are using one method to calculate the area for 3 different types
    fn area(&self) -> f64 {
        match *self {
            Shape::Rectangle {width, height} => (width * height) as f64, // the as f64 is basically casting the output as f64
            Shape::Square(ref s) => (s * s) as f64,
            Shape::Circle(ref r) => 3.14 * (r * r),
        }
    }
}


fn main() {
    // Instantiating an 'emun'
    // Up is basically namespace to Direction hence we use '::'
    let u = Direction::Up( Point {x: 0, y: 1});
    let k = u.match_direction();
    let x = k.destruct(); // This will get rid of the UpKey("") from UpKey("Pressed W") and only show Pressed W...
    let r = Shape::Rectangle{width: 10, height: 70};
    let s = Shape::Square(12);
    let c = Shape::Circle(4.5);

    println!("{}", x);

    let ar = r.area();
    println!("{}", ar);

    let asq = s.area();
    println!("{}", asq);

    let ac = c.area();
    println!("{}", ac);

    let a = 10;
    let b = &a; // v is assigned to a reference of u...this is the equivalent to the next line
    let ref c = a; // Same as the line of code above

    if c == b {
        println!("They are equal!");
    }
}