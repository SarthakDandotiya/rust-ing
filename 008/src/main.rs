// Traits & Generic Types
// Traits a essentially a way for us to share defined beahiour over multiple sets of um...data
// Triats are very similar to interfaces in other languages.
// To create a trait we use the 'trait' keyword.

trait Shape {
    fn area(&self) -> u32;
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

fn main() {
    let c = Circle { radius: 100.132};
    let r = Rectangle { x: 30, y: 20};
    println!("{} {}", c.area(), r.area());
}
