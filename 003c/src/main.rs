// Copying
fn cop(a: i32, b: i32) {
    println!("{}", a + b);
}

fn main() {
    // Defining 2 integers
    let a = 32;
    let b = 45;

    // Passing the 2 integers into this function
    cop(a, b);
    
    // Now unlike earlier where we had the string values, the variables don't get unallocated in this main() funtion...they still exist here, thats because they arent being moved instead they are being copied as they exist on the stack and not the heap
    // So we copy them so that they exist in the scope of both functions

    println!("We have a: {} and b: {}", a, b);
}