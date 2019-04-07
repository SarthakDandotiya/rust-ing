// We can also use generics inside of functions
use std::fmt;
fn p<T: fmt::Debug>(x: T) { // Shorthand for deriving the fmt::Debug trait for our generic T
    println!("{:?}", x);
}

// We can alo use generics inside of implementation blocks
struct A<T> {
    x: T,
}
impl <T> A<T> {
    fn item(&self) -> &T {
        &self.x
    }
}

// We can also use generics as this...
struct A<u, v> {
    x: U,
    y: V,
}

struct A<u, v> {
    x: U,
    y: V,
}

fn main() {
    p(10);
    p(String::from("String!"));

    let a = A {x: "Hello"};
    a.item   
}
