fn take(v: Vec<i32>) {
    println!("We took v: {}", v[10] + v[100]);
}

fn main() {
    let mut v = Vec::new(); // Creating a Vector of a dynamic size

    // Putting data into the Vector
    for i in 1..1000 {
        v.push(i);
    }

    take(v); // When we call take() we tranfer the ownership from the main() function to the take() function and we never return the ownership back to the main() function, this is what is called 'Moving' because we are moving the resource from one function to another
    
    // println!("{}", v[0]);
    println!("Finished");
}
