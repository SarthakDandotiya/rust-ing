// Traits and Generic Types

// Traits are essentialy a way for us to define different behaviour.
// Traits are very similar to interfaces in other languages. They allow us to define what a funtion should look like and allow us to implement that funtion to various different data types.

// To create a trait we juat uae the "trait" keyword
trait Shape {
    fn area (&self) -> u32;
}

struct Rectangle {
    x: u32,
    y: u32,
}

struct Circle {
    radius: f64,
}

impl Shape for Rectangle {
    fn area(&self) -> u32 {
        self.x * self.y
    }
}

impl Shape for Circle {
    fn area(&self) -> u32 {
        (3.141 * self.radius * self.radius) as u32
    }
}

// We also have the 'derive' annotation that we can use to implement various different tratis

fn main() {
    // Instantiate our structs
    let c = Circle { radius: 100.123 };
    let r = Rectangle { x: 30, y: 20 };
    println!("{} {}", c.area(), r.area());
}
