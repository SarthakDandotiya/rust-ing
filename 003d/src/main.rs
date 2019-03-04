// Borrowing
// With the concept of Ownership comes the concept of Borrowing

// There are multiple ways to think about borrowing...
// Essentially borrowing lets us have multiple references to write reuses while adhereing to the single owner, single place of responsibility role.
// They are sort of similar to pointers in other languages like C.
// Reference is also an object.
// Mutable references are moved and immutable references are copied.
// When a reference is dropped, the borrow ends essentially the reference dissapears from the scope.

// In simple cases references behave just like when we are moving owenership between one function and another function

fn re(v: Vec<i32>) -> Vec<i32> { //Takes in a vector and returns a vector
    println!("{}", v[120] + v[111]);
    v
}

fn borrow1(v: &Vec<i32>) {
    println!("{}", (*v)[10] + (*v)[12]); // *v is a pointer to the reference of the vector
}

fn borrow2(v: &Vec<i32>) {
    println!("{}", v[10] + v[11]);
}

fn main() {
    let mut v = Vec::new(); // Make a mutable vector

    // Pushing a 1000 values in the vector
    for i in 1..1000 {
        v.push(i);
    }

    v = re(v); // We transfer ownership to the re() function and it then transfers ownership back

    println!("Still own v: {} {}", v[0], v[1] );

    borrow1(&v); // Takes in a reference to the vector
    println!("Still own v: {} {}", v[0], v[1] );

    borrow2(&v);
    println!("Still own v: {} {}", v[0], v[1] );
}