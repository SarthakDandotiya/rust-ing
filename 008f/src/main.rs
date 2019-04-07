// Now lets talk about generics...
struct Square<T> {
    x:T,
}

// Generics are useful for reducing code duplications.
fn main() {
    let s = Square{ x: 10 };
    let s = Square{ x: 1.0 };
    let s = Square{ x: "Hello" };
    let s = Square{ x: 'c' };
}
