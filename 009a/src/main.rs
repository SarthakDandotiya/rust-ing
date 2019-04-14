// Closures, Box Pointers and Iterators

// A pointer is a variable that contains an adress in memory rather than a actual value, so it points a value hence the name
// Smart poniters on the other hand are data structures that act like a pointer but they have additional metadata
// We'll look at a smart pointer called Box Pointer
// It allows to allocate a piece of data to the heap every primitive in Rust is automatically allocated to the stack, if we wrap it in the Box, it will be allocated to the heap

enum List {
    Cons(i32, Box<List>),
    End,
}

use List::Cons;
use List::End;

fn main() {
    // We can create a box pointer by directly calling the namespace and then the method new(), we put the value we want within it.
    // let b = Box::new(10);
    // println!("b = {}", b);

    // So probably the best example that we can use for explaining the box type is to implement whats called a cons list
    // Cons list is a data structure that comes from the Lips programming language and all of its dialects

    let l = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(End))))));

}
